---
id: SPEC-LINEAR-ALGEBRA-SCALE-CANCEL
title: "Spec 65 §20 cancels a shared summand from vector and matrix sums, but no law cancels a shared nonzero scaling factor, so a client holding vector_scale n (Suc k) xs = vector_scale n (Suc k) ys cannot conclude xs = ys without reasoning entry by entry. Rule from a checked D0 at zero trust that scaling by a successor factor is injective on vectors and on matrices, and state both with a seed that separates a successor factor from a zero one"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). §20 has sum cancellation; scaling has unit, zero, compose and distribution laws but no cancellation. Steward-filed per COORDINATION section 2."
---

# A nonzero scaling factor cancels

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports two public checked laws.
- For every `n`, `k`, `xs` and `ys` at `Vec Nat n`: if `vector_scale n
  (Suc k) xs` equals `vector_scale n (Suc k) ys`, then `xs` equals `ys`.
- For every `m`, `n`, `k`, `a` and `b`: the same holds for `matrix_scale m
  n (Suc k)`.

Both hold at zero trust, and spec 65 states them by their public types.

## Settled inputs (main `1b5e4183e`; anchors re-checked at release)

- **The operations.** `vector_scale` (`NatMatrix.ken.md:57`) is `map (mul
  factor)` over `Vec Nat n`. `matrix_scale` (`:532`) maps `vector_scale`
  over the rows.
- **The neighbours.** §20's `vector_add_cancel_right` (`:962`) and
  `matrix_add_cancel_right` (`:1144`) are the shape to follow: dimension
  binders before the colon, then the inputs and the sole equality premise.
  The scaling unit, zero, compose and distribution laws are at `:858` to
  `:1002` and `:1575` to `:1719`.
- **The gap.** No law takes a scaled equality back to the inputs.
- **The vocabulary.** `Data.Numeric.Nat.Arithmetic` exports `cancel_r for
  mul` (`:234`): `Equal Nat (mul a (Suc k)) (mul b (Suc k)) → Equal Nat a
  b`. `vector_scale` multiplies on the left, `mul factor x`, so the proof
  needs a commutation or a left form.
- **Spec and tests.** Spec 65 ends with §20, so the new section is §21.
  The seed directory is `conformance/stdlib/linear-algebra/`. The test
  pattern is `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §21, the two catalog laws with any private helper, any generic
left-cancellation law in `Nat.Arithmetic` that the proof needs, a seed, a
package test and the status lines. No §1 to §20 operation or law type
moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-ADD-CANCEL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the names, the binder order, and the placement after §20's two laws;
  - whether the factor is written `Suc k`, or as a general factor with a
    nonzero premise;
  - whether a left form of `cancel_r for mul` is a public
    `Nat.Arithmetic` law or a private `NatMatrix` helper;
  - every consumer root of an exported `NatMatrix` law, swept by the
    sibling `vector_add_cancel_right` across `catalog/`, `conformance/`,
    `crates/`, `spec/` and `library/`, so the candidate's scope is
    complete.
- **AC-1.** §21 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block holds a generic client. From a
  hypothesis `vector_scale n 2 xs = vector_scale n 2 ys`, it concludes
  `xs = ys` through the vector law, and the matrix law likewise. Its
  refused block, on a fresh accepted prefix, takes `xs = [1, 2]` and
  `ys = [3, 4]` at factor `Zero`. The premise holds there by `Refl`,
  since both sides are `[0, 0]`, but the claimed `xs = ys` refuses. The
  accepted and refused forms differ only in the factor.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise
  beyond a nonzero factor and the scaled equality.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §20 operation or law type.

Checks: 4 fired. I read `vector_scale`, `matrix_scale` and `cancel_r for
mul` on `1b5e4183e`. The vector obligation is a `Vec` induction whose
head step is `cancel_r` after commuting `mul`, and the matrix obligation
maps it over the rows. Both are writable in the delivered vocabulary; the
route is AC-0's to rule. 3 fired. The sibling `vector_add_cancel_right`
has consumers in `NatMatrix`, the add-cancel seed, the
`cat_linear_algebra_nat_matrix.rs` package test, spec 65 and
SPEC-PROGRESS. AC-0 sweeps the same roots. 8 fired. At factor `Zero`, the
premise is true and the conclusion false, so a law stated without the
successor factor would be unsound. The refused row fails exactly where
that premise is dropped.
