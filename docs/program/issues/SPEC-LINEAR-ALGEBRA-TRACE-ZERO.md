---
id: SPEC-LINEAR-ALGEBRA-TRACE-ZERO
title: "Spec 65 gives the trace of the identity (§18, trace_identity) and the transpose of the zero matrix (§13, transpose_zero), and §22 concludes a zero matrix from a zero Gram trace, but no law gives the trace of the zero matrix itself: a client that sums a diagonal of zeros cannot rewrite it to Zero without unfolding trace by induction. Rule from a checked D0 at zero trust that trace n (matrix_zero n n) is Zero for every n, and state it with an open seed that separates Zero from the dimension"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). trace_identity gave the trace of the multiplicative unit; this gives it for the additive one. Steward-filed per COORDINATION section 2."
---

# The trace of the zero matrix is zero

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public checked law. For every
`n` it states `Equal Nat (trace n (matrix_zero n n)) Zero`. It holds at
zero trust, and spec 65 states it by its public type.

## Settled inputs (main `25ed19d5a`; anchors re-checked at release)

- **The operations.**
  - `trace` (`NatMatrix.ken.md:541`) recurses on the dimension, adding
    the corner entry to the trace of the trailing minor.
  - `matrix_zero m n` (`:533`) is `repeat (Vec Nat n) m (vector_zero n)`.
- **The argument, for AC-0 to check.** Three public laws compose with no
  induction:
  - `matrix_scale_zero` (`:1815`) gives `matrix_scale n n Zero a =
    matrix_zero n n`; take `a = matrix_zero n n`.
  - `trace_scale` (`:2064`) gives `trace n (matrix_scale n n Zero a) =
    mul Zero (trace n a)`.
  - `mul` recurses on its second argument, so `mul Zero t` does not reduce
    for an open `t`. The public `proof zero_l for mul`
    (`Nat/Arithmetic.ken.md:82`) closes it to `Zero`.
  AC-0 may instead prove it by induction on `n` if that is shorter or
  avoids a citation.
- **No such law exists.** The trace laws are `trace_add`, `trace_scale`,
  `trace_transpose`, `trace_identity`, `trace_product_comm` and
  `matrix_zero_of_trace_gram_zero`. None states the trace of
  `matrix_zero`, and nothing in `catalog/` or `spec/` does.
- **Spec and tests.**
  - Spec 65 ends with §23 (`65-linear-algebra.md:1952`), so the new
    section is §24, unless AC-0 places it in §18 beside `trace_identity`.
  - The seed directory is `conformance/stdlib/linear-algebra/`, beside
    `seed-matrix-trace.md`.
  - The test pattern is `crates/ken-elaborator/tests/cat_linear_algebra_trace.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The spec 65 section, the catalog law, a seed, the package test rows and the
status lines. No existing operation or law type moves, there is no new
operation, and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-GRAM-TRACE-ZERO.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder and the placement;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any
    count-bearing roster sentence.
- **AC-1.** The spec section states the law by its public type. CV and the
  Architect review it, with a Decision.
- **AC-2.** The seed has an open row and its twin, each on a fresh accepted
  prefix.
  - The accepted row, for an open `n`, states `Equal Nat (trace n
    (matrix_zero n n)) Zero` by the law.
  - The refused twin states the same trace equal to `n`, the
    `trace_identity` form, and must refuse.
  - The two rows differ only in the claimed result. A closed row at `n =
    Zero` cannot separate them, so the open pair is the discriminator.
    AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing operation or law type.

Checks: 4 fired. I read `trace`, `matrix_zero`, `matrix_scale_zero`,
`trace_scale` and `mul` with its public `zero_l` on `25ed19d5a`; `mul`
recurses on its second argument, so the zero-left step needs the cited law.
3 fired. I listed every public trace law in NatMatrix and grepped
`catalog/` and `spec/` for a trace of `matrix_zero`, and found none. 8
fired. For an open `n` the trace is `Zero` and not `n`, so a law in the
`trace_identity` form fails the refused twin, and a closed `n = Zero` row
alone would not.
