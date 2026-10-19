---
id: CAT-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY
title: "Algorithm.LinearAlgebra.NatMatrix exports spec 65 section 3's laws: matrix_vector of a product applies its factors in turn, transpose reverses a product, and dot_scale is promoted from private. Proved as checked terms at zero trust, from the Architect's D0 development"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY (routed 92bd5582f) states section 3 and records its laws as checked in the Architect's D0 scratch development, not yet exported. Same shape as CAT-LINEAR-ALGEBRA-PRODUCT-LAWS. Steward-filed per COORDINATION section 2."
---

# NatMatrix exports the product compatibility laws

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `matrix_vector_product`,
`transpose_product` and `dot_scale` with exactly spec 65 §3's public
types, each a checked proof.

## Settled inputs (spec 65 §3, from `92bd5582f`)

- **The laws.** §3 gives the three public theorem types. `dot_scale` is
  the existing private lemma, promoted without a second spelling or
  proof body.
- **The proofs.** The Architect's D0 for the spec node
  (`evt_6twckbx699w90` and its thread `thr_230tj8k6cekm1`) checked them
  on the landed base with no Axiom; `trusted_base()` was 109 before and
  after.
- **Package.** `catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`,
  with seed `conformance/stdlib/linear-algebra/seed-nat-matrix.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three exports in `NatMatrix.ken.md`, and test rows in
`cat_linear_algebra_nat_matrix.rs` that root-load each new name and run
the seed's new §3 cases.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the three names as package-owned and transparent, and runs the seed's
  §3 oracles, identity control and refused unreversed-order block.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The strict-resolution census row for
  `Algorithm.LinearAlgebra.NatMatrix` is unchanged.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `transpose_product` stated in the unreversed order is
    kernel-rejected;
  - `vector_scale` ignoring its factor fails the public `dot_scale` proof
    and the seed's `vector_scale_three_oracle` (the seed has no `dot_scale`
    row; foundation QA `evt_68yhdem6h8268`);
  - `transpose` reversing its row order fails the `transpose_product`
    non-square oracle.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §3's.
- A census row, or a consumer of `NatMatrix` outside this package,
  changes.
