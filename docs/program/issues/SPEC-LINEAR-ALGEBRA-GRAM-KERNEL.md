---
id: SPEC-LINEAR-ALGEBRA-GRAM-KERNEL
title: "Spec 65 proves that a zero Gram trace forces a zero matrix (§22) and that a vector with zero self-dot is zero, but no law says that a vector the Gram matrix sends to zero is sent to zero by the matrix itself: a client solving with the normal equations cannot conclude that a null vector of transpose a times a is a null vector of a without re-deriving the adjoint argument. Rule from a checked D0 at zero trust that matrix_vector (transpose a times a) v = zero gives matrix_vector a v = zero, and state it with a seed whose accepted row is open"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). NatMatrix's Gram laws cover the trace and symmetry; none covers the Gram matrix's null vectors. Steward-filed per COORDINATION section 2."
---

# A null vector of the Gram matrix is a null vector of the matrix

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public checked law. For every
`m`, `n`, `a : Vec (Vec Nat n) m` and `v : Vec Nat n`, from

`Equal (Vec Nat n) (matrix_vector n n (matrix_product n m n (transpose m n
a) a) v) (vector_zero n)`

it states

`Equal (Vec Nat m) (matrix_vector m n a v) (vector_zero m)`.

It holds at zero trust, and spec 65 states it by its public type.

## Settled inputs (main `a6b760ed0`; anchors re-checked at release)

- **The argument, for AC-0 to check.** Every step is a public NatMatrix
  law.
  - `matrix_vector_product` (`NatMatrix.ken.md:453`) rewrites the premise to
    `matrix_vector n m (transpose m n a) (matrix_vector m n a v)` equal to
    zero.
  - `transpose_adjoint` (`:1466`), at `x = v` and `y = matrix_vector m n a
    v`, turns the self-dot of `matrix_vector m n a v` into `dot n v` of that
    zero vector.
  - `dot_zero_right` (`:1078`) makes that dot `Zero`, and
    `vector_zero_of_dot_self_zero` (`:1089`) closes the law.
- **No such law exists.** NatMatrix has 67 public theorems. Its Gram laws
  are `matrix_zero_of_trace_gram_zero` (`:2271`) and `transpose_gram`
  (`:2304`). Neither concerns the Gram matrix applied to a vector.
- **Spec and tests.**
  - Spec 65 ends with §24 (`65-linear-algebra.md:2033`), so the new section
    is §25.
  - The seed directory is `conformance/stdlib/linear-algebra/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_linear_algebra_dot_self_zero.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 65 §25, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §24 operation or law type moves,
and there is no kernel or surface change. The converse, and the
`a a^T` form, are out of scope unless AC-0 shows they follow without a
second export.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-GRAM-TRACE-ZERO.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the law needs any private helper;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §25 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `v`, at a fixed closed `a` such as the
    `1 × 2` matrix `[[1, 0]]`, with the premise as a hypothesis. AC-0 shows
    that the row's closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    claims the stronger conclusion `Equal (Vec Nat n) v (vector_zero n)`.
    That claim must refuse: at `[[1, 0]]`, `v = [0, 1]` meets the premise
    and is not zero.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the one stated.
- The proof needs a new public export outside NatMatrix; report it rather
  than widening Vector or Nat.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §24 operation or law type.

Checks: 4 fired. I read `matrix_vector_product`, `transpose_adjoint`,
`dot_zero_right` and `vector_zero_of_dot_self_zero` on `a6b760ed0`; the law
composes them. 3 fired. I listed NatMatrix's 67 public theorems and its two
Gram laws, and found none on the Gram matrix's null vectors. 8 fired. A
closed seed row computes without the law (ADD-POINT-EXACT AC-0,
`evt_1b1n2fsh3bvws`), so the witness row is open, and the refused twin
claims `v` itself is zero, which a nonzero null vector refutes.
