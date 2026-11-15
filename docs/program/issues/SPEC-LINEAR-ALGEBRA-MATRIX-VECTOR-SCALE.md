---
id: SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-SCALE
title: "Spec 65 states that matrix_vector commutes with scaling the vector (matrix_vector_homogeneous) but not with scaling the matrix, so a client cannot move a matrix_scale factor out of a matrix-vector application. Rule from a checked D0 at zero trust that matrix_vector of matrix_scale factor a equals vector_scale factor of matrix_vector a, and state it with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md); Geometry is skipped this turn because its only spec file, spec 67, is held in the Z3-blocked SPEC-GEOMETRY-NAT-ORIENTATION candidate. Steward-filed per COORDINATION section 2."
---

# Scaling the matrix scales the product

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public checked law that
`matrix_vector m n (matrix_scale m n factor a) x` equals
`vector_scale m factor (matrix_vector m n a x)` as `Equal (Vec Nat m)`.
It holds at zero trust, and spec 65 states it by its public type.

## Settled inputs (main `7fa8d4e58`)

- **The gap.** §16 (`spec/50-stdlib/65-linear-algebra.md:1355`) adds
  `matrix_scale` with eight laws, among them `matrix_product_scale`
  for products. No law applies a scaled matrix to a vector. The
  vector-side law exists: `matrix_vector_homogeneous`
  (`NatMatrix.ken.md:1267`; spec `:588`).
- **The operations.** `matrix_scale` is `map (vector_scale n factor)`
  over the rows (`NatMatrix.ken.md:525`). `matrix_vector` is `map
  (λrow. dot n row vector)` (`:64`). `vector_scale` is `map (mul
  factor)` (`:50`).
- **The bridge exists.** `dot_scale` (`:173`) gives `Equal Nat (dot n
  (vector_scale n x u) c) (mul x (dot n u c))`, which is the head row
  of the law.
- **Seeds and tests.** The seed sits beside
  `conformance/stdlib/linear-algebra/seed-matrix-scale.md`. The test
  pattern is `crates/ken-elaborator/tests/cat_linear_algebra_matrix_scale.rs`.
  Spec 65 ends at §16, so the new section is §17.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §17, the catalog law with any private helper, a seed, a
package test and the status lines. No §1 to §16 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-MUL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name, binder order (following
  `matrix_product_scale`), and placement in the export order.
- **AC-1.** §17 states the ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at `m = n = 2`,
  `a = [[1, 2], [3, 4]]`, `x = [1, 1]`, factor `3`, giving `[9, 21]`.
  Its refused block, on a fresh accepted prefix and in the same form
  as an accepted row, claims the scaled application equals the
  unscaled `matrix_vector m n a x`, `[3, 7]`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §16 operation or law type.

Checks: 4 fired: the obligation was written out: on `VNil` both sides
are `VNil`; on `VCons k row rest` the left head is `dot n
(vector_scale n factor row) x`, which `dot_scale` turns into `mul
factor (dot n row x)`, the right head by `map` on `VCons`, and the
tails close by the induction hypothesis under `cong` on `VCons`. 3
fired: `matrix_scale` occurs in NatMatrix, its seed, two tests and
spec 65; grep covered `catalog/`, `crates/`, `conformance/`, `spec/`,
`examples/` and `library/`, and no held candidate touches these paths.
8 fired: a law that drops the factor accepts the AC-2 refused row; the
real operation gives `[9, 21]` against `[3, 7]`, so it refuses it.
