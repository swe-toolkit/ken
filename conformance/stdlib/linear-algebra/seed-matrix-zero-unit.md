# Matrix zero and the unit of matrix addition

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §12, in Spec child
`cc148000520a6d02831c55fb7bf8d373ca01c7e`.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-MATRIX-ZERO-UNIT.md`.

Status: authored; not executed. Section 12 quantifies over every matrix
shape and vector. This seed applies the three public laws to a nonzero
2 × 2 matrix and to a zero-row matrix, and separately checks a closed
matrix-zero value. Those finite rows do not replace the generic checked
proofs.

## Accepted shared client

The rows of `a` are `[1,2]` and `[2,1]`. `empty` has zero rows and
`two` columns; `u` is the length-two vector `[1,2]`.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_add, matrix_zero, matrix_vector, vector_zero,
   matrix_add_zero_left, matrix_add_zero_right,
   matrix_vector_zero_matrix)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))

fn rows2 (r0 : Vec Nat two) (r1 : Vec Nat two)
    : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one r0
    (VCons (Vec Nat two) Zero r1 (VNil (Vec Nat two)))

const a : Vec (Vec Nat two) two = rows2 (pair one two) (pair two one)
const empty : Vec (Vec Nat two) Zero = VNil (Vec Nat two)
const u : Vec Nat two = pair one two

theorem matrix_add_zero_left_nonzero :
  Equal (Vec (Vec Nat two) two)
    (matrix_add two two (matrix_zero two two) a) a =
  matrix_add_zero_left two two a

theorem matrix_add_zero_right_nonzero :
  Equal (Vec (Vec Nat two) two)
    (matrix_add two two a (matrix_zero two two)) a =
  matrix_add_zero_right two two a

theorem matrix_vector_zero_matrix_nonzero :
  Equal (Vec Nat two)
    (matrix_vector two two (matrix_zero two two) u) (vector_zero two) =
  matrix_vector_zero_matrix two two u

theorem matrix_add_zero_left_empty :
  Equal (Vec (Vec Nat two) Zero)
    (matrix_add Zero two (matrix_zero Zero two) empty) empty =
  matrix_add_zero_left Zero two empty

theorem matrix_add_zero_right_empty :
  Equal (Vec (Vec Nat two) Zero)
    (matrix_add Zero two empty (matrix_zero Zero two)) empty =
  matrix_add_zero_right Zero two empty

theorem matrix_vector_zero_matrix_empty :
  Equal (Vec Nat Zero)
    (matrix_vector Zero two (matrix_zero Zero two) u) (vector_zero Zero) =
  matrix_vector_zero_matrix Zero two u

theorem matrix_zero_two_by_two_value :
  Equal (Vec (Vec Nat two) two)
    (matrix_zero two two)
    (rows2 (pair Zero Zero) (pair Zero Zero)) = Refl

theorem matrix_add_zero_left_same_form_control :
  Equal (Vec (Vec Nat two) two)
    (matrix_add two two (matrix_zero two two) a) a = Refl
```

### stdlib/linear-algebra/matrix-zero-unit-laws-closed

- spec: §12.1.
- promise class: durable invariant.
- given: the nonzero `2 × 2` matrix `a` and the zero-row matrix
  `empty`.
- expect: both matrix-add unit laws apply at `two two a` and
  `Zero two empty`; the zero-matrix action law applies at `two two u`
  and `Zero two u`.
- why: the six applications cover both unit orientations and the
  matrix-fixed action at positive and zero row counts.

### stdlib/linear-algebra/matrix-zero-value

- spec: §12.1–§12.2.
- promise class: durable invariant.
- given: `two` and the independently constructed expected matrix
  `[[0,0],[0,0]]`.
- expect: `matrix_zero two two` equals that expected matrix with
  `Refl`.
- why: the expected rows are constructed without using `matrix_zero`.

### stdlib/linear-algebra/matrix-zero-unit-same-form-control

- spec: §12.2.
- promise class: durable invariant.
- given: `a` from the accepted client.
- expect: `matrix_add two two (matrix_zero two two) a` equals `a`
  using `Refl`.
- why: this is the true same-form control for the isolated refusal;
  the sum and proof form stay fixed.

## Isolated refused claim

Start from a fresh copy of the entire accepted client, including the
same-form control, and append only the false theorem below. Its left
endpoint and `Refl` proof form are identical to the accepted control;
only the claimed right-hand side changes. The sum is `a`, not the zero
matrix, so the false claim must be refused at its own `Refl`, rather
than at an earlier import or application.

### stdlib/linear-algebra/reject-zero-matrix-as-additive-result

- spec: §12.2.
- promise class: durable invariant.
- given: the accepted client with `a = [[1,2],[2,1]]`.
- expect: refuse the claim that adding `matrix_zero` to `a` yields
  `matrix_zero`.
- control: the accepted same-form control proves the identical sum
  equals `a` with `Refl`.
- why: the false claim differs only in its right-hand side and must
  not pass because of a proof-shape or fixture failure.

Append only this theorem to a fresh copy of the accepted client:

```ken
theorem reject_zero_as_sum :
  Equal (Vec (Vec Nat two) two)
    (matrix_add two two (matrix_zero two two) a)
    (matrix_zero two two) = Refl
```

## Evidence boundary

Architect's AC-0 `evt_1dgts6nkb7ky2` reports that the proposed
catalog declarations and generic clients checked in scratch on base
`f53d7cfb5bee6a082328d2246827f5ce46cc1130`, with no new `Axiom` and
equal trusted identity sets (109 → 109). Its scratch closed rows
accepted the same-form `Refl` control and refused the false claim at
`Refl`; `Proved` refused both and therefore does not discriminate.
These are Architect-reported D0 results, not execution of this seed.
This seed is authored but unexecuted. No package test, official
conformance-runner execution, CI, publication, landing, or landed trust
measurement is claimed.
