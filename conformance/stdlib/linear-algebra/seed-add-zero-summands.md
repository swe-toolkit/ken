# Zero summands in vector and matrix addition

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §23 (routed child);
Architect AC-0 `evt_218ra2xfqjyj5`.

This authored seed gives black-box clients for the four public zero-sum
laws. Its accepted prefix is AC-0's `prefix.ken`, including all four
open clients. Each refused case starts from a fresh copy of that prefix
and adds only its named theorem.

The §23 and catalog children that add these declarations are not in this
seed's base. The cases have not been checked against an assembled
candidate or run by the official conformance runner. No package-test,
AC-T, CI, publication, landing, or trust result is claimed.

## Accepted client

The open clients apply each public law to abstract vectors or matrices
and a bound zero-sum premise. The closed applications are controls at
shape two; the `one_zero_sum_is_not_zero_sum` theorem identifies the
nonzero sum used by the refused cases.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_zero, matrix_add, matrix_zero,
   vector_add_eq_zero_left, vector_add_eq_zero_right,
   matrix_add_eq_zero_left, matrix_add_eq_zero_right)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1

fn vector2 (x : Nat) (y : Nat) : Vec Nat n2 =
  VCons Nat n1 x (VCons Nat n0 y (VNil Nat))

fn matrix2 (r0 : Vec Nat n2) (r1 : Vec Nat n2) : Vec (Vec Nat n2) n2 =
  VCons (Vec Nat n2) n1 r0 (VCons (Vec Nat n2) n0 r1 (VNil (Vec Nat n2)))

const zero2 : Vec Nat n2 = vector2 n0 n0
const one_zero : Vec Nat n2 = vector2 n1 n0
const zero22 : Vec (Vec Nat n2) n2 = matrix2 zero2 zero2
const one_zero22 : Vec (Vec Nat n2) n2 = matrix2 one_zero zero2

theorem open_vector_left
    (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (sum_zero : Equal (Vec Nat n) (vector_add n xs ys) (vector_zero n))
  : Equal (Vec Nat n) xs (vector_zero n) =
  vector_add_eq_zero_left n xs ys sum_zero

theorem open_vector_right
    (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (sum_zero : Equal (Vec Nat n) (vector_add n xs ys) (vector_zero n))
  : Equal (Vec Nat n) ys (vector_zero n) =
  vector_add_eq_zero_right n xs ys sum_zero

theorem open_matrix_left
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (sum_zero : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_zero m n))
  : Equal (Vec (Vec Nat n) m) a (matrix_zero m n) =
  matrix_add_eq_zero_left m n a b sum_zero

theorem open_matrix_right
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (sum_zero : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_zero m n))
  : Equal (Vec (Vec Nat n) m) b (matrix_zero m n) =
  matrix_add_eq_zero_right m n a b sum_zero

theorem vector_left_zero_by_law : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_left n2 zero2 zero2 Refl

theorem vector_right_zero_by_law : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_right n2 zero2 zero2 Refl

theorem matrix_left_zero_by_law
  : Equal (Vec (Vec Nat n2) n2) zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_left n2 n2 zero22 zero22 Refl

theorem matrix_right_zero_by_law
  : Equal (Vec (Vec Nat n2) n2) zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_right n2 n2 zero22 zero22 Refl

theorem one_zero_sum_is_not_zero_sum
  : Equal (Vec Nat n2) (vector_add n2 one_zero zero2) one_zero =
  Refl
```

### stdlib/linear-algebra/add-zero-summand-open-laws-accept

- spec: §23, all four public zero-sum laws.
- promise class: durable invariant.
- given: the accepted client above, compiled as one fresh source unit.
- expect: accepts. All four open clients type-check through their named
  public laws with the zero-sum premise supplied as a binder. The four
  closed law applications and the reflexive nonzero-sum control also
  type-check.
- why: the open clients exercise each public conclusion at general
  dimensions; the closed zero-sum examples alone would not separate
  generic laws from behavior on the all-zero operands.

## Isolated refused premises

Each case starts from a fresh copy of the entire accepted client above
and appends only its one theorem. Refusal must be at that theorem's
trailing `Refl`, the argument for the zero-sum premise. A failure in the
prefix or at another expression does not satisfy the case.

### stdlib/linear-algebra/ref-xs-vector-add-zero-left-false-sum

- spec: §23 (`vector_add_eq_zero_left`).
- promise class: durable invariant.
- given: the accepted client, with this theorem appended:

```ken
theorem vector_left_zero_by_law_false
  : Equal (Vec Nat n2) one_zero (vector_zero n2) =
  vector_add_eq_zero_left n2 one_zero zero2 Refl
```

- expect: rejects at the trailing `Refl` argument for the sum-equality
  premise.
- control: `vector_left_zero_by_law` in the accepted prefix accepts
  with the same law and a reflexive zero-sum premise.
- why: `vector_add n2 one_zero zero2` reduces to `one_zero`, not
  `vector_zero n2`; `Refl` cannot prove the premise.

### stdlib/linear-algebra/ref-ys-vector-add-zero-left-false-sum-true-conclusion

- spec: §23 (`vector_add_eq_zero_left`).
- promise class: durable invariant.
- given: the accepted client, with this theorem appended:

```ken
theorem vector_left_zero_by_law_false_premise
  : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_left n2 zero2 one_zero Refl
```

- expect: rejects at the trailing `Refl` argument for the sum-equality
  premise.
- control: `vector_left_zero_by_law` accepts with the same conclusion
  and law when both summands are `zero2`.
- why: the conclusion `zero2 = vector_zero n2` is true, but
  `vector_add n2 zero2 one_zero` reduces to `one_zero`, not the zero
  vector. This refusal therefore isolates the false premise rather than
  relying on a false conclusion.

### stdlib/linear-algebra/ref-mat-matrix-add-zero-left-false-sum

- spec: §23 (`matrix_add_eq_zero_left`).
- promise class: durable invariant.
- given: the accepted client, with this theorem appended:

```ken
theorem matrix_left_zero_by_law_false
  : Equal (Vec (Vec Nat n2) n2) one_zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_left n2 n2 one_zero22 zero22 Refl
```

- expect: rejects at the trailing `Refl` argument for the sum-equality
  premise.
- control: `matrix_left_zero_by_law` accepts with the same law and a
  reflexive zero-sum premise.
- why: `matrix_add n2 n2 one_zero22 zero22` reduces to `one_zero22`,
  not `matrix_zero n2 n2`; the first row already distinguishes them.

## Evidence boundary

The accepted and refused outcomes are the seed oracle, grounded in
Architect AC-0 for the routed §23 child and its measured clients. They
have not been executed on the assembled candidate or by the official
conformance runner. This seed does not assert proof-body correctness or exact
`trusted_base()` identities; those belong to the package gate.