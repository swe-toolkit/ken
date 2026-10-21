---
id: SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY
title: "Spec 65 adds vectors but not matrices: a client of NatMatrix can multiply matrices but cannot sum them, so nothing states that a matrix product distributes over a matrix sum. Rule a componentwise matrix sum and its distributivity laws from a checked D0 at zero trust, and state them as a section 4 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65 has vector_add and dot_distrib but no matrix sum, so the product's linearity is unstated. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-GEOMETRY-BOX-AREA. Steward-filed per COORDINATION section 2."
---

# Matrix sums and distributivity

## Objective

Spec 65 states a public componentwise sum of two `m × n` natural
matrices in `Algorithm.LinearAlgebra.NatMatrix`, with checked laws
that distribute `matrix_product` over it on each side.

## Settled inputs (spec 65 §1 to §3, on `main` at `f4fa5c259`)

- **Operations (§1.2).** `vector_add`, `vector_scale`, `dot`,
  `matrix_vector`, `transpose`, `matrix_product` and `identity`, with
  matrices as `Vec (Vec Nat n) m`. There is no matrix sum.
- **Laws.** §1.3 `dot_distrib` expands `dot` over `vector_add` on its
  left argument. §2 has `matrix_identity_left` and
  `matrix_product_assoc`; §3 has `matrix_vector_product`,
  `transpose_product` and `dot_scale`. All are `pub` in
  `catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md`.
- **Limits (§1.4).** Signed scalars, a semiring abstraction, a
  Rational tier, and every `Float` result are outside the contract.
- **Package.** Tested by
  `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`, with
  a census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 65 section stating the matrix sum and each ruled law by its
public type, and a closed conformance seed under
`conformance/stdlib/linear-algebra/`, with the status lines changed in
the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the operation's name and definition, row by row through
    `vector_add`;
  - left and right distributivity of `matrix_product` over the sum,
    with their exact index order;
  - whether `matrix_vector` and `transpose` distribute over the sum,
    and whether either follows from an existing law without a new
    statement;
  - the private helpers the proofs need, including a right-argument
    `dot` distributivity if `dot_distrib` and `dot_comm` do not give it.
- **AC-1.** The section states each ruled operation and law by its
  public type. The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed checks a closed `2 × 2` sum, and left and right
  distributivity on closed non-square operands. It has a refused block
  that claims `matrix_product a (sum b c)` equals the sum of
  `matrix_product a b` and `c`.

## Stop conditions

- A law that needs signed scalars, subtraction, a semiring abstraction
  or a new carrier, unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature, or a change to a §1 to
  §3 operation or law.
- An ambient name in the package beyond its current census row: import
  it explicitly.

Checks: 4 fired: `vector_add`, `dot_distrib`, `matrix_product_assoc`
and `transpose_product` are `pub` in `NatMatrix.ken.md` at
`f4fa5c259`, and no matrix sum exists there. 7 fired: the refused
block drops a product from one side, which a seed checking only shapes
would accept.
