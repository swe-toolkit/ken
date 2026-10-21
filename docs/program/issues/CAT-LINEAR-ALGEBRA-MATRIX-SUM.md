---
id: CAT-LINEAR-ALGEBRA-MATRIX-SUM
title: "Algorithm.LinearAlgebra.NatMatrix exports spec 65 section 4: the row-wise matrix sum and its four laws (left and right product distributivity, matrix_vector distributivity, transpose_add), proved at zero trust from the Architect's D0 development, with the section 4 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY states section 4 (083b4b2ff) from the Architect's checked D0 scratch development, not yet exported. Same shape as CAT-GEOMETRY-BOX-AREA. Steward-filed per COORDINATION section 2."
---

# NatMatrix exports the matrix sum

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `matrix_add`,
`matrix_product_distrib_left`, `matrix_product_distrib_right`,
`matrix_vector_distrib` and `transpose_add` with exactly spec 65 §4's
public types, each law a checked proof.

## Settled inputs (spec 65 §4; Architect D0)

- **The surface.** `matrix_add m n a b` is `zip_with` of `vector_add n`
  over the rows (§4.1). The four laws and their index order are §4.2.
  `dot_distrib_right`, `columns_add_right`, `row_product_add` and
  `zip_prepend_add` stay private; `columns_add` and `zero_width_repeat`
  are reused (§4.3).
- **The D0.** `/workspaces/ken/local/arch-msd-d0/natmatrix-d0.diff` on
  base `b5b423cad`, with `spec_linear_algebra_sum_d0_trust.rs` and the
  refusal and control probes beside it. Checked rc 0, trust identities
  109 → 109, private-import refusal E0012. The census was not run to a
  verdict; this WP owns it.
- **Seed.** The §4 seed under `conformance/stdlib/linear-algebra/`,
  from SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY AC-2. It must be
  on `main` before this WP starts.
- **Package.** `catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`,
  with a census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The five exports and four private helpers in `NatMatrix.ken.md`, with
any package prose that excludes a matrix sum corrected; the spec 65 §4
status lines updated to say the exports are delivered; and a new
`crates/ken-elaborator/tests/cat_linear_algebra_matrix_sum.rs` that
root-loads each new public name and runs the seed. Every seed
execution-status statement changes in the same candidate to say the
package test executes it, with no conformance-runner, CI or landing
claim, and no fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the five names as package-owned and transparent, keeps the four
  helpers private, and runs the seed's positive cases, including its
  direct `2 × 2` sum oracle. Its refused block is pinned to the seed's
  dropped-product proposition, and the same-operand control checks.
  The §1 to §3 suite still passes. The Spec leader reviews the spec/
  status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.LinearAlgebra.NatMatrix` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `matrix_add` built with a row operation other than `vector_add n`
    (for example returning `a`) fails the seed's sum oracle;
  - an unrelated false closed proposition in the refused block fails
    the pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §4's.
- A new public `Nat` or `Vec` lemma, or any change outside
  `NatMatrix`: keep the helpers private, or import an existing public
  lemma.
- A change to a §1 to §3 operation or law, a census row change, or a
  change to a consumer of `NatMatrix` outside this package.

Checks: 4 fired: `natmatrix-d0.diff` adds exactly the five public and
four private names, `columns_add` and `zero_width_repeat` exist in the
package, and the census names `NatMatrix`. 7 fired: AC-3 mutates the
sum itself, which the generic laws alone would not catch.
