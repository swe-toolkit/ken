---
id: SPEC-LINEAR-ALGEBRA-GRAM-TRACE-ZERO
title: "Spec 65 §14 proves that a zero self-dot forces the zero vector, but no law lifts this to matrices: a client holding trace m (matrix_product m n m a (transpose m n a)) = 0 cannot conclude a is the zero matrix, and nothing states that a matrix times its own transpose is symmetric. Rule from a checked D0 at zero trust that the Gram trace vanishes only at the zero matrix and that the Gram product equals its transpose, and state both with a seed that separates a zero matrix from one nonzero entry"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). §14 is the vector form of the property; §18 and §19 give the trace but no matrix-level definiteness. Steward-filed per COORDINATION section 2."
---

# The Gram trace vanishes only at zero

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports two public checked laws.
- For every `m`, `n` and `a : Vec (Vec Nat n) m`: if `trace m
  (matrix_product m n m a (transpose m n a))` equals `Zero`, then `a`
  equals `matrix_zero m n`.
- For the same binders: `transpose m m (matrix_product m n m a (transpose
  m n a))` equals `matrix_product m n m a (transpose m n a)`.

Both hold at zero trust, and spec 65 states them by their public types.

## Settled inputs (main `5bc385c1c`; anchors re-checked at release)

- **The vector form.** `vector_zero_of_dot_self_zero`
  (`NatMatrix.ken.md:1050`) takes `Equal Nat (dot n v v) Zero` to `Equal
  (Vec Nat n) v (vector_zero n)`. `matrix_zero` (`:531`) repeats
  `vector_zero n` over the rows.
- **The trace.** `trace` (`:539`) recurses through `drop_column`. The
  private `trace_row_dot_sum` (`:2455`) already rewrites the trace of a
  row-by-columns product as a sum of dots, and `trace_product_comm`
  (`:2128`) uses it.
- **Symmetry vocabulary.** `transpose_product` (`:492`) and
  `transpose_invol` (`:266`) give the second law in a few steps.
- **Nat vocabulary.** A sum of naturals that is zero has zero summands.
  The D0 names the `Nat.Arithmetic` law it uses, or a private helper.
- **Spec and tests.** Spec 65 ends with §21, so the new section is §22.
  The seed directory is `conformance/stdlib/linear-algebra/`. The test
  pattern is `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §22, the two catalog laws with any private helper, any generic
`Nat.Arithmetic` law the proof needs, a seed, a package test and the
status lines. No §1 to §21 operation or law type moves, and there is no
kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-SCALE-CANCEL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, the binder order, whether the
  diagonal of the Gram product is reached through `trace_row_dot_sum` or
  a new private helper, and whether the zero-sum law is public.
- **AC-1 (seed).** A closed 2-by-2 seed: the zero matrix has Gram trace
  0; a matrix with one entry 1 has Gram trace 1, and applying the first
  law to it must refuse at its false premise. One non-symmetric matrix
  checks the second law.
- **AC-2 (AC-T).** A package test checks both laws from a root load, runs
  the seed fences, and finds the trusted identity sets equal before and
  after. An unused `Axiom` mutation reddens that equality.

## Stop conditions

- The first law needs a premise beyond the trace equality, or holds only
  for square `a`. Report the D0's measured form; do not weaken the law
  silently.
- The proof needs a kernel, surface or trust change.

Checks: 4 fired. On `5bc385c1c` I read `vector_zero_of_dot_self_zero`,
`matrix_zero`, `trace`, `trace_row_dot_sum`, `transpose_product` and
spec 65's last section. 2 fired. The property is matrix definiteness of
the Gram trace. Its fan-in is the §14 vector law, the trace's diagonal
walk, and a natural zero-sum split, and AC-0 names each. 8 fired. The
one-entry seed has trace 1, so a law that ignores its premise is caught.
