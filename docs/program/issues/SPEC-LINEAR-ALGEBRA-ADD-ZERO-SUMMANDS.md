---
id: SPEC-LINEAR-ALGEBRA-ADD-ZERO-SUMMANDS
title: "Spec 65 cancels a common summand (§20) and characterizes the zero vector by its self-dot (§14), but says nothing about a sum that is zero: over Nat there are no additive inverses, so a vector or matrix sum is zero only when both summands are, and a client cannot use that. Rule from a checked D0 at zero trust that vector_add n xs ys = vector_zero n gives xs = vector_zero n, with the matrix analogue, and state it with a seed that separates a zero summand from a nonzero one"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). §14 and §20 gave zero-detection and cancellation; this gives the zero-sum half. Steward-filed per COORDINATION section 2."
---

# A zero sum has zero summands

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports public checked laws. For every
`n`, `xs` and `ys`, from `Equal (Vec Nat n) (vector_add n xs ys)
(vector_zero n)` the law concludes `Equal (Vec Nat n) xs (vector_zero n)`,
and the matrix law does the same with `matrix_add` and `matrix_zero`. Both
hold at zero trust, and spec 65 states them by their public types.

## Settled inputs (main `b36f84e03`; anchors re-checked at release)

- **The operations.**
  - `vector_add` (`NatMatrix.ken.md:56`) is `zip_with ... add`.
  - `matrix_add` (`:526`) is `zip_with ... (vector_add n)` over rows.
  - `vector_zero` (`:62`) is `repeat Nat n Zero`, and `matrix_zero` is at
    `:531`.
- **The argument, for AC-0 to check.**
  - Vector, by induction on `n`, in the shape of `vector_add_cancel_right`
    (`:964`) and its private cons helper (`:4314`). At the head,
    Arithmetic's `eq_zero_l for add` (`Arithmetic.ken.md:68`) gives the zero
    component. The tail uses the IH.
  - Matrix, by induction on the rows, applying the vector law to each row.
  - The right summand follows by `eq_zero_r for add` (`:74`) in the same
    shape, or by `vector_add_comm` (`:796`). AC-0 rules whether the right
    form is public.
- **Neighbours.**
  - §14 `vector_zero_of_dot_self_zero` (`:1050`) is the precedent for a
    zero-detection law over Nat, and uses `eq_zero_r for add` the same way.
  - §20 `vector_add_cancel_right` and `matrix_add_cancel_right` are the
    precedent for the binder shape.
- **No such law exists.** NatMatrix has no law concluding a summand is
  zero.
- **Spec and tests.**
  - Spec 65 ends with §22 (`65-linear-algebra.md:1896`), so the new section
    is §23.
  - The seed directory is `conformance/stdlib/linear-algebra/`, after
    `seed-add-cancel.md`.
  - The package test is
    `crates/ken-elaborator/tests/cat_linear_algebra_nat_matrix.rs`, or a
    sibling if AC-0 rules one.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §23, the catalog laws with any private helper, a seed, a package
test and the status lines. No §1 to §22 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-ADD-CANCEL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the names, the binder order and the placement;
  - whether both summand forms are public;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the laws by their public types. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed uses `n = 2`.
  - Its accepted block takes `xs = [0, 0]` and `ys = [0, 0]`. It proves the
    sum is zero by `Refl`, and concludes through the law that `xs` is zero.
  - Its refused block, on a fresh accepted prefix and in the same form,
    takes `xs = [1, 0]`. The sum's premise `Refl` refuses, because the sum
    is `[1, 0]`.
  - The two rows differ only in `xs`'s first component.
  - AC-0 may replace this seed with a stronger discriminator, if one
    separates the law from a law without the zero-sum premise.
- **AC-T.** The package test compares `trusted_base()` before and after
  NatMatrix as identity sets, and that comparison fails when the package
  adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The laws need an `Axiom`, a kernel or surface change, or a premise beyond
  the zero sum.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `vector_add`, `matrix_add`, `vector_zero`,
`matrix_zero`, `vector_add_cancel_right` and its cons helper,
`vector_zero_of_dot_self_zero`, and Arithmetic's `eq_zero_l` and
`eq_zero_r for add` on `b36f84e03`. The argument uses only those and
induction. 3 fired. I grepped NatMatrix and the catalog for a law
concluding a summand is zero, and found none. 8 fired. The refused row's
sum is `[1, 0]`, not zero, so its premise refuses. AC-0 is invited to
supply a row that also separates a law that drops the premise.
