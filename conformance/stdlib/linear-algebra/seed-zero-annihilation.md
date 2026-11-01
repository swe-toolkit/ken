# Zero annihilation for dot, scaling, and matrix application

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §11, Spec child
`261efd4ccaf39693467d5ec0b2a6b72ca42804d9`.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-ZERO-ANNIHILATION.md`.

Status: authored; not executed. The laws quantify over all dimensions.
This seed applies each law with open dimensions and at `two`; its closed
vectors do not replace the generic theorem obligations.

## Accepted shared client

Let `u = [1,2]`, `zero2 = [0,0]`, and let `a` have rows `[1,2]`
and `[2,1]`. The entries of `a` are nonzero. Its image of `u` is
`[5,4]`, while its image of `zero2` is `zero2`.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_zero, vector_scale, dot, matrix_vector,
   dot_zero_right, vector_scale_zero_vector, matrix_vector_zero)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four

fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))

const u : Vec Nat two = pair one two
const zero2 : Vec Nat two = pair Zero Zero
const a : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (pair one two)
    (VCons (Vec Nat two) Zero (pair two one) (VNil (Vec Nat two)))

theorem dot_open (n : Nat) (xs : Vec Nat n)
  : Equal Nat (dot n xs (vector_zero n)) Zero =
  dot_zero_right n xs

theorem scale_open (n : Nat) (factor : Nat)
  : Equal (Vec Nat n)
      (vector_scale n factor (vector_zero n)) (vector_zero n) =
    vector_scale_zero_vector n factor

theorem matrix_vector_open
    (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m)
  : Equal (Vec Nat m)
      (matrix_vector m n rows (vector_zero n)) (vector_zero m) =
    matrix_vector_zero m n rows

theorem dot_two
  : Equal Nat (dot two u (vector_zero two)) Zero =
    dot_zero_right two u

theorem dot_two_value
  : Equal Nat (dot two u (vector_zero two)) Zero = Proved

theorem scale_two
  : Equal (Vec Nat two)
      (vector_scale two two (vector_zero two)) (vector_zero two) =
    vector_scale_zero_vector two two

theorem scale_two_value
  : Equal (Vec Nat two)
      (vector_scale two two (vector_zero two)) zero2 = Refl

theorem matrix_vector_two
  : Equal (Vec Nat two)
      (matrix_vector two two a (vector_zero two)) (vector_zero two) =
    matrix_vector_zero two two a

theorem matrix_vector_nonzero_value
  : Equal (Vec Nat two) (matrix_vector two two a u) (pair five four) = Refl

theorem matrix_vector_zero_control
  : Equal (Vec Nat two) (matrix_vector two two a zero2) zero2 = Refl
```

### `stdlib/linear-algebra/zero-annihilation-laws-open`

- spec: §11.1.
- promise class: durable invariant.
- given: arbitrary dimensions and vectors, rows, and natural scale factor.
- expect: the three public laws apply at their stated generic types;
  each also applies to the closed length-two fixture.
- claimed: the laws hold without premises for every natural dimension.
- gap: applying the exports does not check their generic proof bodies.

### `stdlib/linear-algebra/zero-annihilation-laws-closed`

- spec: §11.1–§11.2.
- promise class: durable invariant.
- given: `u = [1,2]`, `zero2 = [0,0]`, and the nonzero matrix
  `a = [[1,2],[2,1]]`.
- expect: the matrix sends `u` to `[5,4]` and `zero2` to `zero2`;
  the three law applications at `two` check.
- claimed: dot with the right zero vector is zero, scaling the zero
  vector preserves it, and every matrix sends a zero vector to zero.
- gap: these instances do not establish the three generic proofs.

## Isolated refused claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg` below. The accepted `matrix_vector_zero_control` keeps the
matrix, result type, expected `zero2`, and `Refl` proof form fixed; the
false claim changes only the vector input from `zero2` to `u`. The
accepted `matrix_vector_nonzero_value` independently checks that the
same input maps to `[5,4]`, not `zero2`.

### `stdlib/linear-algebra/reject-nonzero-matrix-vector-as-zero`

- spec: §11.2.
- promise class: durable invariant.
- given: `a = [[1,2],[2,1]]`, `u = [1,2]`, and `zero2 = [0,0]`.
- expect: refuse `seed_neg` at its own `Refl` with
  ``Refl expects an `Eq`-shaped goal``; the same-form `zero2` control
  accepts.
- claimed: the public zero-input law does not say a matrix sends a
  nonzero vector to zero.
- why: the computed image is `[5,4]`, not `zero2`. `Refl` separates
  this false equality from the same-form accepted control; `Proved`
  refuses that true control too.
- gap: the isolated refusal does not establish the generic laws.

```ken
theorem seed_neg : Equal (Vec Nat two) (matrix_vector two two a u) zero2 = Refl
```

## Evidence boundary

The Spec child and new catalog exports are not on the authoring base
`a39d385c8`. Architect AC-0 checked the generic proofs and measured
the proof-form pair in scratch; it did not run this seed.

Foundation reports that its package-owned test
`cat_linear_algebra_nat_matrix` passed 7/7 at assembled candidate
`fb7e0934bb7e464f3ff11330a0b391d5f782c097`. The test elaborated the
accepted and refused Ken source blocks in that tip's seed file, whose
blob matched seed child
`06f5aaa022c4e36f9548228615aafda9b58401b5` exactly, against
root-loaded `NatMatrix`. This is package-test elaboration, not official
conformance-runner execution. This package-test evidence does not
establish official conformance-runner execution; none is claimed here.
No CI, publication, landing, or landed-trust result is claimed.
