---
id: SPEC-LINEAR-ALGEBRA-TRACE
title: "Spec 65 gives square Nat matrices an identity, transpose, sum and scaling but no trace, so a client cannot state the sum of a diagonal or its linearity. Rule from a checked D0 at zero trust a public trace on Vec (Vec Nat n) n with its additive, scaling, transpose and identity laws, and state them with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md:376); NatMatrix has no trace. Steward-filed per COORDINATION section 2."
---

# The trace of a square matrix

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public `trace` on square
matrices `Vec (Vec Nat n) n`, the sum of the diagonal, with four public
checked laws at zero trust:

- **additive:** `trace (matrix_add a b)` equals `add (trace a) (trace b)`;
- **scaling:** `trace (matrix_scale x a)` equals `mul x (trace a)`;
- **transpose:** `trace (transpose a)` equals `trace a`;
- **identity:** `trace (identity n)` equals `n`.

Spec 65 states the operation and the laws by their public types.

## Settled inputs (main `77a4e9907`)

- **The carrier.** Matrices are `Vec (Vec Nat n) m` (`NatMatrix.ken.md`).
  `identity` (`:81`) recurses on `n` through `prepend_zero`, and
  `transpose` (`:69`) recurses on rows through `zip_with` and `prepend`.
  `matrix_add` (`:517`) and `matrix_scale` (`:525`) are row-wise.
- **The gap.** No `trace` or diagonal-sum operation or law is declared in
  `catalog/packages`, `spec/` or `conformance/`. The word "diagonal"
  occurs only in prose that names no such operation: the identity's
  description (`:27`, `65-linear-algebra.md:127`), the `vector_identity`
  proof note (`:3483`), identity seeds, and the kernel's size-change
  matrices (amended, Steward, on Foundation's report
  `evt_4wz1rsxdhnft7`).
- **Neighbouring laws.** `transpose_add` (`:655`), `transpose_scale`
  (`:1685`), `transpose_identity` (`:1422`) and `transpose_invol`
  (`:257`) are the transpose laws a trace proof may reuse.
- **Spec and tests.** Spec 65 ends at §17
  (`65-linear-algebra.md:1483`), so the new section is §18. The test
  pattern is `crates/ken-elaborator/tests/cat_linear_algebra_transpose_adjoint.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §18, the operation and its laws with any private helper, a seed,
a package test and the status lines. No §1 to §17 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-TRANSPOSE-ADJOINT. If AC-0 measures that the
transpose law alone exceeds one useful turn, the Architect rules a split:
the operation with its additive, scaling and identity laws first, then
transpose as a successor.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the definition of `trace`: its recursion, and how it takes the
    diagonal head and the tail minor;
  - the names and binder orders;
  - the proof route for each law;
  - the split, if any.
- **AC-1.** §18 states the operation and each ruled law by its public
  type. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block evaluates `trace` on a closed 3×3
  matrix with distinct diagonal and off-diagonal entries, and applies
  each law at closed arguments.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `trace` claimed equal to the sum of the first row;
  - the scaling law claimed as `add x (trace a)`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §17 operation or law type.
- `trace` cannot be defined on `Vec (Vec Nat n) n` without a new
  dependent-vector operation outside `NatMatrix`.

Checks: 4 fired. Each law's `Suc` step splits the matrix into its first
row and the remaining rows. Diagonal head and minor commute with
`matrix_add` and `matrix_scale` row-wise. For transpose, the minor of
the transpose is the transpose of the minor, and that is the step AC-0
measures. 3 fired: the grep for `trace` and `diagonal` covered
`catalog/packages`, `spec/` and `conformance/`. 8 fired: with distinct
off-diagonal entries, the first-row sum differs from the diagonal sum,
and at `x = 3` with a trace other than 0 or 1, `add x` differs from
`mul x`. So
each refused row is false on the closed matrix.
