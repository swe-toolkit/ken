---
id: SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY
title: "Spec 65 states that matrix_vector distributes over a matrix sum, but no law says it is linear in its vector: additive over vector_add and homogeneous over vector_scale. Rule from a checked D0 at zero trust that both hold for every dimension, and state them with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra is next in the layer rotation after Geometry. Same shape as SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY. Steward-filed per COORDINATION section 2."
---

# Matrix-vector product is linear in its vector

## Objective

Spec 65 states, as public checked laws over
`Algorithm.LinearAlgebra.NatMatrix`, that `matrix_vector m n a` maps
`vector_add n x y` to the `vector_add m` of the two images, and
`vector_scale n k x` to `vector_scale m k` of the image, for every `m`
and `n`.

## Settled inputs (spec 65 §1 to §5, main `a4626cae5`)

- **The operations.** `matrix_vector` maps `λrow. dot n row vector`
  over the rows (`NatMatrix.ken.md:54-57`). `vector_add` is
  `zip_with add` and `vector_scale` is `map (mul factor)` (`:39-43`).
- **No law in the vector argument.** §4.2's `matrix_vector_distrib`
  adds the matrices and keeps one vector. §3's `matrix_vector_product`
  composes two matrices. No section varies the vector.
- **Why it holds.** Each row's image is a `dot` with the vector in
  second position. `dot_comm` with `dot_distrib` gives additivity in
  that position, and `dot_comm` with `dot_scale` gives homogeneity
  (`:82`, `:116`, `:163`). The matrix laws then follow row by row
  through `map`.
- **Status row.** The spec 65 row in `spec/SPEC-PROGRESS.md` still says
  the §5 catalog exports are not delivered.
  `CAT-LINEAR-ALGEBRA-SUM-COMM-ASSOC` has merged, and the exports are
  at `NatMatrix.ken.md:650-757`. This candidate's own status edit
  replaces that row.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-matrix-sum-distributivity.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §6 in spec 65 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/linear-algebra/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names, the public types, and which helpers stay private (for example
  a second-position `dot` distribution).
- **AC-1.** The new section states each ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to a closed,
  non-square `2 × 3` matrix and distinct length-3 vectors, with each
  image also computed independently by `Refl`. Its refused block is
  pinned to one false proposition on the same inputs. One example is
  `matrix_vector` of the sum equal to the image of `x` alone.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §5 law or operation.
