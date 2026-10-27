---
id: CAT-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY
title: "Algorithm.LinearAlgebra.NatMatrix exports spec 65 section 6: matrix_vector_additive and matrix_vector_homogeneous, proved at zero trust from the Architect's D0, with the section 6 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY]
blocks: []
github: null
origin: "Catalog half of SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY (spec candidate be64bf168, Decision dec_3y1xx3s3jym56). Same shape as CAT-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY. Steward-filed per COORDINATION section 2."
---

# NatMatrix exports matrix-vector linearity

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `matrix_vector_additive`
and `matrix_vector_homogeneous` with exactly spec 65 §6's public types,
as checked proofs.

## Settled inputs (spec 65 §6 at `be64bf168`, D0 on `aa6370449`)

- **The surface.** §6.1 states `matrix_vector_additive (m n a x y)` and
  `matrix_vector_homogeneous (m n factor a x)`, both with no premise.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/linalg-mv-d0/d0.patch`
  (sha256 `fb38570a…`; ruling `ruling.md` beside it): check and fmt rc
  0, probe test 2/2, `trusted_base()` 109 before and after with equal
  GlobalId sets, no `Axiom`. Its hunks place the two laws after
  `vector_add_assoc` and the private `dot_scale_right` after
  `dot_distrib_right`. No import changes. Its file
  `arch_linalg_mv_d0.rs` is scratch and does not ship.
- **One private helper.** `dot_scale_right` stays private; additivity
  reuses the existing private `dot_distrib_right`.
- **The consumers.** `cat_linear_algebra_nat_matrix.rs`
  (`public_operations_and_generic_laws_preserve_provider_trust_set`)
  pins the exact visible export set. The Architect measured it red by
  exactly the two new names; it goes from twenty-six to twenty-eight
  with its count wording. The package prose counts "eighteen algebraic
  laws" and "twenty-six public names" (`NatMatrix.ken.md:4`, `:2415`,
  `:2421`), and the D0 does not edit it. The other NatMatrix consumers
  the Architect inspected list named entries only.
- **Seed.**
  `conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two exports and helper in `NatMatrix.ken.md` with its count prose
corrected, the export set in `cat_linear_algebra_nat_matrix.rs`
updated, and a new
`crates/ken-elaborator/tests/cat_linear_algebra_matrix_vector_linearity.rs`
that root-loads both names and runs the seed. Once the spec candidate
has landed, update spec 65 §6's status lines and the seed's
execution-status statements in the same candidate to say the package
test checks them. Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  both laws as package-owned and transparent, refuses importing
  `dot_scale_right`, and runs the seed's accepted client. Seed cases
  are selected by theorem name, not by fence count. Its refused block
  is pinned to `seed_neg`'s false `[7,6] = [1,2]`, and
  `additive_closed_refl` checks as the same-form control. The existing
  `cat_linear_algebra_*` binaries pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.LinearAlgebra.NatMatrix` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `matrix_vector_homogeneous`'s head step with `dot_distrib_right`
    in place of `dot_scale_right` fails check;
  - `seed_neg`'s right side replaced by the true `vec2 seven six` fails
    the pin.

## Stop conditions

- A law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than §6's.
- A change to an existing §1 to §5 law or operation, or a census row
  change.

Checks: 3 fired: the exact-set pin and prose counts are from the
Architect's measured consumer inventory (`ruling.md`, six consumers).
8 fired: the second AC-3 mutant turns the refusal into an accepted true
claim, so a pin that only checks that some block was refused stays
green.
