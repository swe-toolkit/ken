---
id: SPEC-LINEAR-ALGEBRA-ZERO-ANNIHILATION
title: "Spec 65 §10 gives the zero vector as the unit of vector_add and states that scaling by Zero and a dot product with a left zero vector both yield zero, but states no law that the zero vector is absorbed: dot with a right zero vector, scaling the zero vector, and a matrix applied to the zero vector. A client must recompose dot_comm, vector_scale_zero and matrix_vector_homogeneous to rewrite any of them. Rule from a checked D0 at zero trust and state the laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra is next in the layer rotation after Geometry. Steward-filed per COORDINATION section 2."
---

# The zero vector is absorbed

## Objective

Spec 65 states, as public checked laws over
`Algorithm.LinearAlgebra.NatMatrix`, for all `m`, `n`, `factor`, `xs`
and `a`:

- `Equal Nat (dot n xs (vector_zero n)) Zero`;
- `Equal (Vec Nat n) (vector_scale n factor (vector_zero n))
  (vector_zero n)`;
- `Equal (Vec Nat m) (matrix_vector m n a (vector_zero n))
  (vector_zero m)`.

## Settled inputs (main `3122196f6`)

- **The public neighbours exist.** `vector_zero` (`NatMatrix.ken.md:46`)
  and the §10 laws `vector_add_zero_left`, `vector_add_zero_right`,
  `vector_scale_zero` and `dot_zero_left` (`:859` to `:906`). `dot_comm`
  (`:85`) and `matrix_vector_homogeneous` (`:969`) are public.
- **Not yet stated.** No public theorem concludes any of the three
  equations. The private `matrix_vector_zero_prefix` (`:1627`) and
  `dot_zero_prefix` (`:1616`) are prefix helpers for the identity
  proofs, not these laws.
- **The proof-form evidence so far** (spec 65 §10.2,
  `65-linear-algebra.md:946` to `:957`). At a closed length a
  `Vec Nat` equality accepts `Refl`, while the true `Equal Nat`
  dot-zero claim refuses `Refl` and accepts `Proved`.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-vector-zero-unit.md`, whose `one`, `two`, `pair`, `u` and
  `zero2` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §11 in spec 65 stating the ruled laws by their public types, the
catalog theorems in `NatMatrix`, a closed conformance seed under
`conformance/stdlib/linear-algebra/`, the public-inventory pin
advanced, and the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem names, argument order and placement, which
  helpers stay private, and whether a law is derived from the public
  neighbours or proved by its own match. It confirms that no §1 to §10
  operation or law type changes, and amends any §10 sentence that the
  new laws make false.
- **AC-1.** §11 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies all three laws at
  `n = two` to `u` and to a two-by-two matrix with nonzero entries.
  - Its refused block pins the matrix applied to `u` as equal to
    `zero2`. Its true value has nonzero components, so the claim is
    false.
  - Its accepted same-form control is the same matrix applied to
    `zero2`, equal to `zero2`, so the refusal is the vector operand, not
    the form.
  - The D0 rules the proof form that separates the two rows. Do not
    presume `Refl` or `Proved`: §10.2 shows that the answer depends on
    the goal's shape.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: each law is written from public `vector_zero`,
`dot_comm`, `vector_scale_zero` and `matrix_vector_homogeneous`, each
read at its line; the package was grepped for a public theorem
concluding these equations, with no hit. 8 fired: the refused row and
its control share the matrix and the claim form and differ only in the
vector operand, whose image is nonzero for `u` and zero for `zero2`.
