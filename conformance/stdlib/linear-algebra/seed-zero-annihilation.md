# Zero annihilation in natural linear algebra

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §11.

Status: authored; not officially conformance-run. The accepted client
applies all three public checked laws at arbitrary dimensions and at
length two. Closed values fix a nonzero matrix's output independently
of the laws. They do not replace the generic proof obligations.

## Accepted shared client

Take `u = [1,2]` and the matrix with rows `[1,2]` and `[2,1]`.
The matrix contains nonzero entries and sends `u` to `[5,4]`,
whereas it sends the zero vector to `[0,0]`.

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

theorem dot_open (n : Nat) (v : Vec Nat n)
  : Equal Nat (dot n v (vector_zero n)) Zero = dot_zero_right n v

theorem scale_open (n : Nat) (f : Nat)
  : Equal (Vec Nat n) (vector_scale n f (vector_zero n)) (vector_zero n) =
  vector_scale_zero_vector n f

theorem mv_open (m : Nat) (n : Nat) (b : Vec (Vec Nat n) m)
  : Equal (Vec Nat m) (matrix_vector m n b (vector_zero n)) (vector_zero m) =
  matrix_vector_zero m n b

theorem dot_two : Equal Nat (dot two u (vector_zero two)) Zero =
  dot_zero_right two u

theorem scale_two : Equal (Vec Nat two)
    (vector_scale two two (vector_zero two)) (vector_zero two) =
  vector_scale_zero_vector two two

theorem mv_two : Equal (Vec Nat two)
    (matrix_vector two two a (vector_zero two)) (vector_zero two) =
  matrix_vector_zero two two a

theorem matrix_nonzero_value : Equal (Vec Nat two)
    (matrix_vector two two a u) (pair five four) = Refl

theorem matrix_zero_value : Equal (Vec Nat two)
    (matrix_vector two two a zero2) zero2 = Refl

theorem zero_vector_value : Equal (Vec Nat two) (vector_zero two) zero2 = Refl

theorem matrix_zero_same_form_control : Equal (Vec Nat two)
    (matrix_vector two two a zero2) zero2 = Refl
```

### stdlib/linear-algebra/zero-annihilation-open

- spec: §11.
- promise class: durable invariant.
- given: arbitrary `n`, `m`, scalar factor, vector and matrix.
- expect: all three public generic laws check without a premise.
- claimed: a right zero vector annihilates dot, scaling preserves the
  zero vector, and a matrix sends the zero vector to its output zero.
- the gap: applying the exported laws does not check their bodies;
  the package checker checks their proofs separately.

### stdlib/linear-algebra/zero-annihilation-two

- spec: §11.
- promise class: durable invariant.
- given: the stated vector and nonzero two-by-two matrix.
- expect: each public law applies at length two. The independently
  constructed `[5,4]` checks the nonzero input's matrix result;
  the zero input checks as `[0,0]` with `Refl`.
- claimed: the two inputs have different matrix results for these
  fixed values.
- the gap: these examples alone cannot prove the arbitrary-index
  equalities.

## Isolated refused claim

Start from a fresh copy of the full accepted client and append only
this false theorem. It uses the same matrix, output `zero2`, result
type and `Refl` proof form as `matrix_zero_same_form_control`; only
the input vector changes from `zero2` to `u`.

### stdlib/linear-algebra/reject-nonzero-matrix-input-as-zero

- spec: §11.
- promise class: durable invariant.
- given: the accepted prefix, where the matrix maps `u` to `[5,4]`.
- expect: the false equality refuses at its own `Refl`.
- control: the accepted same-form zero-input theorem checks with
  `Refl`, excluding an inadmissible proof form as the explanation.
- the gap: this false closed claim does not replace the public proof.

```ken
theorem seed_neg : Equal (Vec Nat two)
    (matrix_vector two two a u) zero2 = Refl
```

## Evidence boundary

The seed is authored, not officially conformance-run. Its package-owned
roots-loader test is candidate-local checking, not runner execution.
No CI, publication, landing or landed trust measurement is claimed.
