---
id: SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT
title: "Spec 65 relates transpose to matrix products and sums, but no law moves a matrix across a dot product. Rule from a checked D0 at zero trust that the transpose is the adjoint of matrix_vector for every dimension, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra is next in the layer rotation after Geometry. Same shape as SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY. Steward-filed per COORDINATION section 2."
---

# Transpose is the adjoint of matrix-vector application

## Objective

Spec 65 states, as a public checked law over
`Algorithm.LinearAlgebra.NatMatrix`, that for every `m`, `n`, matrix
`a : Vec (Vec Nat n) m`, `x : Vec Nat n` and `y : Vec Nat m`:

    dot m (matrix_vector m n a x) y
      = dot n x (matrix_vector n m (transpose m n a) y)

as structural `Equal Nat`.

## Settled inputs (spec 65 §1 to §6, main `9d4ba1e83`)

- **The operations.** `matrix_vector` maps `λrow. dot n row vector`
  over the rows (`NatMatrix.ken.md:54-57`). `transpose` zips `prepend`
  of the head row onto the transposed tail (`:59-64`). `dot` recurses on
  aligned heads (`:45-52`).
- **No law crosses a dot product.** §3's `transpose_product` and §4's
  `transpose_add` keep the transpose on matrices, and §6 varies only the
  vector. No section relates `dot` to `transpose`.
- **Why it should hold.** It is a natural-semiring identity with no
  subtraction. On a matrix `row :: tail`, the transposed image of
  `y0 :: ys` is `y0` times `row` plus the transposed tail's image of
  `ys`. Then `dot_distrib`, `dot_scale` and `dot_comm` (`:82`, `:116`,
  `:163`) give the step, and induction gives the tail. Whether the
  step needs a new private helper is the D0's to measure.
- **Status row.** The spec 65 row in `spec/SPEC-PROGRESS.md` still says
  the §6 catalog exports are absent. `CAT-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY`
  has merged (`544a46307`). This candidate's own status edit replaces
  that row.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-matrix-vector-linearity.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 65 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/linear-algebra/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type and argument order, and which helpers stay
  private.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed,
  non-square `2 × 3` matrix with distinct length-3 `x` and length-2 `y`,
  with both sides also computed independently by `Refl`. Its refused
  block is pinned to one false proposition on the same inputs that has
  the right type, for example the right side with `y`'s two entries
  swapped.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §6 law or operation.

Checks: 4 fired: the law is written in the catalog's own signatures
(`dot`, `matrix_vector`, `transpose` at `NatMatrix.ken.md:45-64`), and
both sides type-check by index (`Vec Nat m` against `y`, `Vec Nat n`
against `x`). 6 fired: "no law crosses a dot product" was measured by
reading every `pub theorem` in `NatMatrix.ken.md` (20 exports) and
spec 65 §1 to §6; none mentions both `dot` and `transpose`.
