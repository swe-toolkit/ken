---
id: SPEC-LINEAR-ALGEBRA-MATRIX-SCALE
title: "Spec 65 scales vectors by a natural factor but has no matrix scaling, so a client cannot state k·A, nor that it distributes over matrix sums, composes, commutes with transpose and passes through a matrix product. Rule from a checked D0 at zero trust a public matrix_scale with those laws, and state them with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md); vector scaling (spec 65 section 9) has no matrix counterpart. Steward-filed per COORDINATION section 2."
---

# Scaling a matrix by a natural factor

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `matrix_scale` on an `m × n`
matrix, scaling every row with `vector_scale`. Its laws hold at zero
trust, and spec 65 states them by their public types.

## Settled inputs (main `d2bf34648`)

- **The gap.** A grep for `matrix_scale` over `catalog/`, `spec/`,
  `conformance/` and `crates/` finds 0 hits.
- **The row operation.** `vector_scale n factor xs` is
  `map Nat Nat n (mul factor) xs`
  (`catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md:49`).
  `matrix_add` maps `vector_add` over rows (`:516`), so `matrix_scale`
  maps `vector_scale n k` over rows in the same shape.
- **Row laws already public.** `vector_scale_distrib` (`:835`),
  `vector_scale_compose` (`:876`, `mul x y` on the left),
  `dot_scale` (`:172`) and `dot_scale_right` (`:239`).
- **The product route.** `matrix_product` maps the private
  `row_product n p right` over the left rows (`:75`). `row_product`
  dots each row with the columns of `transpose n p right` (`:1767`).
  So the left law reduces to `dot_scale` per entry. The right law
  needs transpose to commute with scaling first, then
  `dot_scale_right`.
- **Seeds and tests.** Seeds sit under
  `conformance/stdlib/linear-algebra/`. The test pattern is
  `crates/ken-elaborator/tests/cat_linear_algebra_matrix_zero_unit.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A new spec 65 section, the catalog operation and laws, a seed and the
status lines. No §1 to §15 operation or law type moves, and no kernel or
surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the law set and names, starting from:
  - distribution over `matrix_add`: `k·(A + B) = k·A + k·B`;
  - composition: `(x·y)·A = x·(y·A)`, in `vector_scale_compose`'s
    orientation;
  - transpose: `transpose (k·A) = k·(transpose A)`;
  - product, left and right: `(k·A)·B = k·(A·B) = A·(k·B)`.
  It also rules which helpers stay private and whether any existing
  private helper must become public, naming the owner of that change.
- **AC-1.** The new section states `matrix_scale` and each ruled law by
  its public type. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block evaluates `matrix_scale 3` on a
  closed non-square `2 × 3` matrix with distinct non-zero entries, and
  applies each law to a closed instance. Its refused block pins, in the
  same form as an accepted row, the matrix with only the first row
  scaled, and the matrix with `3` added to every entry. Each refusal
  sits on a fresh accepted prefix.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §15 operation or law type.

Checks: 4 fired: `vector_scale`, `matrix_add`, `matrix_product`,
`row_product` and the four row laws were read at `d2bf34648` on the cited
lines, and the left product law's obligation per entry is `dot_scale`'s
statement. 3 fired: the `matrix_scale` grep covered `catalog/`, `spec/`,
`conformance/` and `crates/`, with 0 hits. 8 fired: an operation that
ignores its factor satisfies distribution, composition, transpose and
both product laws, and scaling only the first row satisfies all but
transpose. So the closed evaluation row is the discriminator, and the
refused twins are those two mutants. Factor `3` is used because `3·x`
and `x + 3` differ at every natural `x`, and a factor of `2` agrees with
the added-constant mutant at `x = 2`.
