---
id: CAT-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT
title: "Algorithm.LinearAlgebra.NatMatrix exports spec 65 section 7: transpose_adjoint, proved at zero trust from the Architect's D0, with the section 7 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT]
blocks: []
github: null
origin: "Catalog half of SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT (spec candidate 915ed15d9, Decision dec_7v54rydca75x7). Same shape as CAT-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY. Steward-filed per COORDINATION section 2."
---

# NatMatrix exports the transpose adjoint law

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `transpose_adjoint` with
exactly spec 65 §7's public type, as a checked proof.

## Settled inputs (spec 65 §7 at `915ed15d9`, D0 on `54f68d3cc`)

- **The surface.** §7 states `transpose_adjoint (m n a x y)`:
  `Equal Nat (dot m (matrix_vector m n a x) y)
  (dot n x (matrix_vector n m (transpose m n a) y))`, with no premise.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/la-transpose-adjoint-d0/d0.patch`
  (sha256 `aadeb73d…`): build, check, fmt and test rc 0, probe test 2/2,
  `trusted_base()` 109 before and after, no `Axiom`. Its two
  `NatMatrix.ken.md` hunks place the law after
  `matrix_vector_homogeneous` and the private helpers after
  `dot_scale_right`. Its file `arch_la_adjoint_d0.rs` is scratch and
  does not ship.
- **Two private helpers.** `dot_zero_columns` and `transpose_cons_image`
  are new and private. `dot_scale_right` and `dot_distrib_right` are
  reused and stay private.
- **The consumers.**
  - `cat_linear_algebra_nat_matrix.rs` pins the exact visible export set
    and says "twenty-eight" (`:157`, `:227`). It goes to twenty-nine.
  - The package prose says "twenty algebraic laws" (`NatMatrix.ken.md:4`)
    and "twenty-eight public names across eight operations and twenty
    laws" (`:2512`, `:2520`). These go to twenty-one and twenty-nine.
- **Seed.** `conformance/stdlib/linear-algebra/seed-transpose-adjoint.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- The export and both helpers in `NatMatrix.ken.md`, with its count
  prose corrected.
- The export set in `cat_linear_algebra_nat_matrix.rs` updated.
- A new `crates/ken-elaborator/tests/cat_linear_algebra_transpose_adjoint.rs`
  that root-loads the law and runs the seed.
- Spec 65 §7's status lines, the spec 65 row in `spec/SPEC-PROGRESS.md`
  and the seed's execution-status statements, updated to say the package
  test checks them. No other status row changes.

Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass.
  - The test root-loads the law as package-owned and transparent.
  - It refuses importing either private helper.
  - It runs the seed's accepted client, with cases selected by theorem
    name, not by fence count.
  - Its refused block is pinned to `seed_neg`'s false 35 = 37, with the
    same-form accepted control checking.
  - The existing `cat_linear_algebra_*` binaries pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.LinearAlgebra.NatMatrix` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `transpose_adjoint`'s cons step with `transpose_cons_image` replaced
    by a `Refl` fails check;
  - `seed_neg`'s right side replaced by the true 35 fails the pin.

## Stop conditions

- The law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than §7's.
- A change to an existing §1 to §6 law or operation, or a census row
  change.

Checks: 3 fired: the exact-set pin and prose counts were grepped at
`54f68d3cc` (`cat_linear_algebra_nat_matrix.rs:157,227`,
`NatMatrix.ken.md:4,2512,2520`). 8 fired: the second AC-3 mutant turns
the refusal into an accepted true claim, so a pin that only checks that
some block was refused stays green.
