---
id: SPEC-LINEAR-ALGEBRA-ADD-CANCEL
title: "Spec 65 gives vector and matrix addition commutativity, associativity and a zero unit, but no cancellation law, so a client holding xs + zs = ys + zs cannot conclude xs = ys without inducting over the vectors itself, although natural addition cancels (add_cancel_r). Rule from a checked D0 at zero trust that vector_add and matrix_add cancel on the right at every dimension, and state both with a seed whose refused row applies the law to a false premise"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). vector_add and matrix_add have no cancellation law. Steward-filed per COORDINATION section 2."
---

# Vector and matrix addition cancel on the right

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports two public checked laws. For
every `n` and `xs`, `ys`, `zs : Vec Nat n`, `Equal (Vec Nat n)
(vector_add n xs zs) (vector_add n ys zs)` gives `Equal (Vec Nat n) xs
ys`. For every `m`, `n` and `a`, `b`, `c : Vec (Vec Nat n) m`, the same
holds for `matrix_add m n`. Both hold at zero trust, with the sum
equality as their only premise. Spec 65 states them by their public
types.

## Settled inputs (main `f3ecab2b9`; anchors re-checked at release)

- **The operations.** `vector_add` is `zip_with Nat Nat Nat n add`
  (`NatMatrix.ken.md:53`). `matrix_add` is `zip_with` of `vector_add n`
  over rows (`:523`).
- **The neighbours.** `vector_add_comm` (`:793`), `vector_add_assoc`
  (`:831`), `vector_add_zero_left` and `_right` (`:930`, `:946`), and the
  matrix forms (`:724`, `:762`, `:1077`, `:1108`). `matrix_add_comm`
  already lifts `vector_add_comm` row by row. The private `vector_cons_eq`
  (`:2327`), `row_head` and `row_tail` (`:4147`, `:4152`) are the cons
  congruence and projections a cancellation proof inverts.
- **The scalar law.** `add_cancel_r (a b) : (c : Nat) → Equal Nat (add a
  c) (add b c) → Equal Nat a b` is public in
  `Data.Numeric.Nat.Arithmetic` (`Arithmetic.ken.md:310`). The package
  imports `add`, `mul` and two distributivity laws from that module
  today, not `add_cancel_r`.
- **The gap.** `git grep -i cancel` over the package and spec 65 has no
  hit.
- **Spec and tests.** Spec 65 ends with §19
  (`spec/50-stdlib/65-linear-algebra.md:1662`), so the new section is
  §20. The seed directory is `conformance/stdlib/linear-algebra/`. The
  test pattern is
  `crates/ken-elaborator/tests/cat_linear_algebra_trace_product_comm.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §20, the two catalog laws with any private helper, the
`add_cancel_r` import, a seed, a package test and the status lines. No §1
to §19 operation or law type moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-TRACE-PRODUCT-COMM.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0 and no `Axiom`. It rules the names, the
  binder order, whether the matrix law reuses the vector law row by row,
  and whether left cancellation is owed as well or left derivable through
  `vector_add_comm`.
- **AC-1.** §20 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies each law to a closed
  instance, a 2-vector and a 2-by-2 matrix, with the sum equality
  discharged by `Refl` and the conclusion stated.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix: the vector law applied to `xs = [1, 2]`, `ys =
  [2, 1]`, `zs = [0, 0]` with `Refl` for the premise. The two sums differ,
  so the premise fails to check and the conclusion `[1, 2] = [2, 1]` is
  never reached. The row differs from the accepted one only in `ys`.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate, and no Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the sum equality.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §19 operation or law type.

Checks: 3 fired. I grepped `NatMatrix.ken.md` and spec 65 for `cancel`
(no hits), and the catalog for cancellation laws: `add_cancel_r` is the
public Nat law. 4 fired. I wrote the obligation in the delivered
vocabulary: at `VCons`, the premise is a cons equality, and `row_head`
and `row_tail` under `cong` give `add x z = add y z`, closed by
`add_cancel_r`, and the tail equality, closed by recursion, joined by
`vector_cons_eq`. At `VNil` both sides are `VNil`. 8 fired. The refused
row's premise is false, so it refuses exactly when the law is applied
without a true sum equality.
