# A natural Gram matrix is symmetric and its trace detects zero

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §22.

## stdlib/linear-algebra/gram-trace-and-symmetry

- spec: `spec/50-stdlib/65-linear-algebra.md` §22.
- given: the selective-import client below, first at open dimensions, then
  at closed square and rectangular inputs.
- expect: accepts. The zero matrix has Gram trace zero, and putting a one
  in its upper-left entry changes the trace to one. The 1 × 3 zero input
  also admits the zero conclusion. For `skew = [[1,2],[0,1]]`, the
  transpose is `[[1,0],[2,1]]` but the Gram product is `[[5,2],[2,1]]`:
  the product, not its input, is symmetric.
- why: check the public laws without a squareness premise. Literal
  entries and traces give independent value checks, not consequences
  computed using the two new proofs.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix
  (matrix_product,
    matrix_zero,
    trace,
    transpose,
    matrix_zero_of_trace_gram_zero,
    transpose_gram)

theorem open_gram_trace_zero
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m)
    : Equal Nat (trace m (matrix_product m n m a (transpose m n a))) Zero
      → Equal (Vec (Vec Nat n) m) a (matrix_zero m n) =
  matrix_zero_of_trace_gram_zero m n a

theorem open_gram_symmetry
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat m) m)
        (transpose m m (matrix_product m n m a (transpose m n a)))
        (matrix_product m n m a (transpose m n a)) =
  transpose_gram m n a

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const five : Nat = Suc (Suc three)

fn row2 (x : Nat) (y : Nat) : Vec Nat two = VCons Nat one x (VCons Nat Zero y (VNil Nat))

fn mat2 (first : Vec Nat two) (second : Vec Nat two) : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one first (VCons (Vec Nat two) Zero second (VNil (Vec Nat two)))

const zero2 : Vec (Vec Nat two) two = mat2 (row2 Zero Zero) (row2 Zero Zero)

const one_entry : Vec (Vec Nat two) two = mat2 (row2 one Zero) (row2 Zero Zero)

const skew : Vec (Vec Nat two) two = mat2 (row2 one two) (row2 Zero one)

const wide_zero : Vec (Vec Nat three) one =
  VCons
    (Vec Nat three)
    Zero
    (VCons Nat two Zero (VCons Nat one Zero (VCons Nat Zero Zero (VNil Nat))))
    (VNil (Vec Nat three))

theorem zero_gram_trace
    : Equal Nat (trace two (matrix_product two two two zero2 (transpose two two zero2))) Zero =
  Proved

theorem one_entry_gram_trace
    : Equal Nat
        (trace two (matrix_product two two two one_entry (transpose two two one_entry)))
        one =
  Proved

theorem zero_gram_recovers_input : Equal (Vec (Vec Nat two) two) zero2 (matrix_zero two two) =
  matrix_zero_of_trace_gram_zero two two zero2 Proved

theorem wide_gram_recovers_input
    : Equal (Vec (Vec Nat three) one) wide_zero (matrix_zero one three) =
  matrix_zero_of_trace_gram_zero one three wide_zero Proved

theorem skew_transpose_value
    : Equal
        (Vec (Vec Nat two) two)
        (transpose two two skew)
        (mat2 (row2 one Zero) (row2 two one)) =
  Refl

theorem skew_gram_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two skew (transpose two two skew))
        (mat2 (row2 five two) (row2 two one)) =
  Refl

theorem skew_gram_symmetric
    : Equal
        (Vec (Vec Nat two) two)
        (transpose two two (matrix_product two two two skew (transpose two two skew)))
        (matrix_product two two two skew (transpose two two skew)) =
  transpose_gram two two skew
```

## stdlib/linear-algebra/gram-trace-premise-refuses

- spec: `spec/50-stdlib/65-linear-algebra.md` §22.
- given: append this theorem to the accepted client in a fresh environment.
  Compared with `zero_gram_recovers_input`, only the input changes from
  `zero2` to `one_entry` at the same dimensions.
- expect: rejects (kernel `TypeMismatch` in the appended theorem). `Proved`
  cannot inhabit its trace-zero premise; the diagnostic need not isolate
  that token.
- why: the law cannot be used on a nonzero matrix with a positive Gram
  trace.

```ken
theorem refuse_nonzero_gram_trace
    : Equal (Vec (Vec Nat two) two) one_entry (matrix_zero two two) =
  matrix_zero_of_trace_gram_zero two two one_entry Proved
```

## stdlib/linear-algebra/non-symmetric-gram-input-refuses

- spec: `spec/50-stdlib/65-linear-algebra.md` §22.
- given: append this theorem to the accepted client, independently of the
  premise refusal above.
- expect: rejects (kernel `TypeMismatch` in the appended theorem).
- why: the Gram symmetry example must not start with a symmetric input.

```ken
theorem refuse_skew_symmetric : Equal (Vec (Vec Nat two) two) (transpose two two skew) skew =
  Proved
```

These are authored seed cases. Checking them in the package test is not
execution by the official conformance runner.
