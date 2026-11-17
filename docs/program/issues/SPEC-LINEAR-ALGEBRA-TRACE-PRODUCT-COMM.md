---
id: SPEC-LINEAR-ALGEBRA-TRACE-PRODUCT-COMM
title: "Spec 65 §18 gives trace laws for addition, scaling, transpose and identity, but none for a matrix product, so a client cannot exchange the factors under a trace, the step every similarity-invariance argument needs. Rule from a checked D0 at zero trust that for a an m-by-n and b an n-by-m natural matrix, trace m (matrix_product a b) equals trace n (matrix_product b a), and state it with a conformance seed"
status: merged
owner: spec
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md:376). trace has no product law. Steward-filed per COORDINATION section 2."
---

# Trace is invariant under exchanging the factors of a product

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public checked law that, for
every `m`, `n`, `a : Vec (Vec Nat n) m` and `b : Vec (Vec Nat m) n`,
`trace m (matrix_product m n m a b)` equals
`trace n (matrix_product n m n b a)` as `Equal Nat`. It holds at zero
trust with no premise, for rectangular factors, and spec 65 states it by
its public type.

## Settled inputs (main `760b8eccd`)

- **The operations.** `trace` (`NatMatrix.ken.md:534`) adds the minor's
  trace to the corner, removing the first row and column through private
  `drop_column` and `row_head`. `matrix_product` (`:80`) maps private
  `row_product n p right` over the left rows, and `dot` (`:59`) recurses
  on both vectors.
- **The tools.** `transpose_product` (`:487`), `dot_comm` (`:96`),
  `matrix_product_distrib_left` and `_right` (`:544`, `:585`) and
  `trace_transpose` (`:1964`) are public. The trace laws are
  `trace_add` (`:1855`), `trace_scale` (`:1915`), `trace_transpose`
  and `trace_identity` (`:2035`).
- **The gap.** No law in `catalog/packages`, `spec/` or `conformance/`
  relates `trace` to `matrix_product`.
- **Spec and tests.** Spec 65 ends at §18 (`65-linear-algebra.md:1551`),
  so the new section is §19. The test pattern is
  `crates/ken-elaborator/tests/cat_linear_algebra_trace.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §19, the catalog law with any private helper (a double-sum
exchange is expected), a seed, a package test and the status lines. No §1
to §18 operation or law type moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-TRACE. If the D0 shows the law does not fit one
candidate, the Architect rules a split (for example, a private sum
exchange first) in AC-0; size is the Steward's, not an operator question.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name, the binder order and the proof
  route, for example induction on `m` with a sum-exchange helper, or a
  route through `transpose_product` and `trace_transpose`.
- **AC-1.** §19 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed
  arguments:
  - rectangular factors, `a = [[1,2,0],[0,1,3]]` (2 by 3) and
    `b = [[1,0],[2,1],[0,1]]` (3 by 2), where both traces are 9 while
    the products have different sizes;
  - square factors `c = [[1,1],[0,1]]` and `d = [[1,0],[1,1]]`, where
    both traces are 3.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `matrix_product c d` claimed equal to `matrix_product d c`. They are
    `[[2,1],[1,1]]` and `[[1,1],[1,2]]`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §18 operation or law type.

Checks: 4 fired. Both sides are the double sum over `i < m` and `j < n`
of `a[i][j] · b[j][i]`. The left sums rows of `a` first and the right
sums rows of `b` first, so the proof is a sum exchange plus `mul`
commutation, which is why a private helper is expected. 3 fired: I grepped
`catalog/packages`, `spec/` and `conformance/` for `trace` with
`matrix_product` and found none. 8 fired: on the square fixture the two
products differ but their traces are both 3, so the refused row is false
while the accepted trace row is true. They differ only in whether `trace`
is applied. On the rectangular fixture, `a b` is `[[5,2],[2,4]]` and `b a`
has diagonal 1, 5, 3, so both traces are 9.
