---
id: SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY
title: "Spec 65 exports matrix_vector, matrix_product and transpose but states no law that relates them: nothing proves that applying a product to a vector is applying its factors in turn, or that transpose reverses a product. Rule both from a checked D0 at zero trust over natural scalars, and state them as a section 3 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65's laws (sections 1.3 and 2) relate each operation only to itself or to identity. Linear algebra is the roadmap layer after statistics (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-STATISTICS-EVENT-MASS-INCLUSION-EXCLUSION. Steward-filed per COORDINATION section 2."
---

# Products compose with vectors and reverse under transpose

## Objective

Spec 65 states, as checked laws over `Algorithm.LinearAlgebra.NatMatrix`,
that `matrix_vector` of a product is `matrix_vector` of its factors in
turn, and that `transpose` of a product is the product of the
transposes in reverse order.

## Settled inputs (spec 65 §1 and §2, on `main`)

- **Operations (§1.2).** `matrix_vector m n rows vector`,
  `transpose m n rows` and `matrix_product m n p left right` are over
  `Vec (Vec Nat n) m`, with dimensions in their checked types.
- **Laws today.** §1.3: `dot_comm`, `dot_distrib`, `transpose_invol`,
  `vector_identity` and `matrix_identity_right`. §2:
  `matrix_identity_left` and `matrix_product_assoc`, landed in the
  package by `CAT-LINEAR-ALGEBRA-PRODUCT-LAWS` (`bc092bdd5`). No law
  mentions both `matrix_vector` and `matrix_product`, or both
  `transpose` and `matrix_product`.
- **Stale text.** §1.4 and §2 still say the §2 laws are not exported by
  the module, which is no longer true.
- **Package.** `catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`,
  with seed `conformance/stdlib/linear-algebra/seed-nat-matrix.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 65 section stating each ruled law by its public type, the
corrected §1.4 and §2 export text, and a closed conformance seed under
`conformance/stdlib/linear-algebra/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - `matrix_vector m p (matrix_product m n p a b) x` equal to
    `matrix_vector m n a (matrix_vector n p b x)` over every operand;
  - `transpose m p (matrix_product m n p a b)` equal to
    `matrix_product p n m (transpose n p b) (transpose m n a)`;
  - which helpers (such as `dot` against a scaled or summed vector) are
    public laws and which stay private.
- **AC-1.** The section states each ruled law by its public type, and
  §1.4 and §2 are corrected. The Spec leader and CV review it, with a
  Decision.
- **AC-2.** The seed checks both laws on closed non-square operands
  (for example 2×3 and 3×2), with a positive control on the identity,
  and has a refused block that claims the product in the unreversed
  order.

## Stop conditions

- A law that needs subtraction, signed scalars, a semiring abstraction
  or a premise beyond §1's data, unless the D0 rules it as the stated
  premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
