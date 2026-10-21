# Matrix sums and product distributivity

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §4, on main at
`3809ccdd46fab11556ac3387c7a896ed442f8b82`.

The §4 contract is landed at this seed's base, but its catalog exports
are absent. This file is a source/content seed only: neither source
fence has been executed. No shipped-seed execution, CI, publication,
or landing claim is made.

The closed cases use `Nat` matrices with dimensions in their `Vec`
types. They check a direct 2×2 row-wise sum, then both distribution
orientations on non-square products. The false sum-on-the-right
claim (`a(b+c)=ab+c`) uses the same square left factor and right
summands as its accepting control; only the dropped product differs.

## stdlib/linear-algebra/matrix-add-2x2-direct-oracle

- spec: `spec/50-stdlib/65-linear-algebra.md` §4.1.
- promise class: durable invariant.
- given: independently constructed 2×2 matrices `[[1,0],[2,1]]` and
  `[[0,3],[1,1]]`.
- expect: `matrix_add` returns `[[1,3],[3,2]]`, compared with a
  separately constructed expected `Vec`.
- why: the expected matrix does not come from the operation under test.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix (matrix_add, matrix_product)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six
const eight : Nat = Suc seven
const nine : Nat = Suc eight
const ten : Nat = Suc nine

fn vec1 (x : Nat) : Vec Nat one =
  VCons Nat Zero x (VNil Nat)
fn vec2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn vec3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat Zero z (VNil Nat)))

fn matrix_1_2 (row : Vec Nat two) : Vec (Vec Nat two) one =
  VCons (Vec Nat two) Zero row (VNil (Vec Nat two))
fn matrix_1_3 (row : Vec Nat three) : Vec (Vec Nat three) one =
  VCons (Vec Nat three) Zero row (VNil (Vec Nat three))
fn matrix_2_1 (row0 : Vec Nat one) (row1 : Vec Nat one)
    : Vec (Vec Nat one) two =
  VCons (Vec Nat one) one row0
    (VCons (Vec Nat one) Zero row1 (VNil (Vec Nat one)))
fn matrix_2_2 (row0 : Vec Nat two) (row1 : Vec Nat two)
    : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one row0
    (VCons (Vec Nat two) Zero row1 (VNil (Vec Nat two)))
fn matrix_2_3 (row0 : Vec Nat three) (row1 : Vec Nat three)
    : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one row0
    (VCons (Vec Nat three) Zero row1 (VNil (Vec Nat three)))
fn matrix_3_1 (row0 : Vec Nat one) (row1 : Vec Nat one)
    (row2 : Vec Nat one) : Vec (Vec Nat one) three =
  VCons (Vec Nat one) two row0
    (VCons (Vec Nat one) one row1
      (VCons (Vec Nat one) Zero row2 (VNil (Vec Nat one))))

const sum_left_22 : Vec (Vec Nat two) two =
  matrix_2_2 (vec2 one zero) (vec2 two one)
const sum_right_22 : Vec (Vec Nat two) two =
  matrix_2_2 (vec2 zero three) (vec2 one one)
const sum_expected_22 : Vec (Vec Nat two) two =
  matrix_2_2 (vec2 one three) (vec2 three two)

theorem matrix_add_two_by_two_value :
  Equal (Vec (Vec Nat two) two)
    (matrix_add two two sum_left_22 sum_right_22)
    sum_expected_22 = Refl

const b23 : Vec (Vec Nat three) two =
  matrix_2_3 (vec3 one zero two) (vec3 zero one one)
const c23 : Vec (Vec Nat three) two =
  matrix_2_3 (vec3 two one zero) (vec3 one zero three)
const b_plus_c_23 : Vec (Vec Nat three) two =
  matrix_2_3 (vec3 three one two) (vec3 one one four)

theorem matrix_add_two_by_three_value :
  Equal (Vec (Vec Nat three) two)
    (matrix_add two three b23 c23)
    b_plus_c_23 = Refl
```

## stdlib/linear-algebra/matrix-product-distrib-left-1x2-2x3

- spec: `spec/50-stdlib/65-linear-algebra.md` §4.2.
- promise class: durable invariant.
- given: a 1×2 left factor and two 2×3 right summands.
- expect: both sides of `a(b+c) = ab+ac` produce `[[5,3,10]]`.
- measured: a closed relation and an independent value oracle for
  the product of the left factor with the right-hand sum.
- why: the output is 1×3, and all three columns depend on the input
  entries.

```ken
const factor_12 : Vec (Vec Nat two) one =
  matrix_1_2 (vec2 one two)
const left_expected_13 : Vec (Vec Nat three) one =
  matrix_1_3 (vec3 five three ten)

theorem left_sum_product_value :
  Equal (Vec (Vec Nat three) one)
    (matrix_product one two three factor_12 (matrix_add two three b23 c23))
    left_expected_13 = Refl

theorem matrix_product_distrib_left_closed :
  Equal (Vec (Vec Nat three) one)
    (matrix_product one two three factor_12 (matrix_add two three b23 c23))
    (matrix_add one three
      (matrix_product one two three factor_12 b23)
      (matrix_product one two three factor_12 c23)) = Refl
```

## stdlib/linear-algebra/matrix-product-distrib-right-2x3-3x1

- spec: `spec/50-stdlib/65-linear-algebra.md` §4.2.
- promise class: durable invariant.
- given: two 2×3 left summands and a 3×1 right factor.
- expect: both sides of `(a+b)c = ac+bc` produce `[[7],[7]]`.
- measured: a closed relation and an independent value oracle for
  the product of the left-hand sum with the right factor.
- why: the 2×1 result is non-square and both rows use all three
  columns of the right factor.

```ken
const factor_31 : Vec (Vec Nat one) three =
  matrix_3_1 (vec1 one) (vec1 two) (vec1 one)
const right_expected_21 : Vec (Vec Nat one) two =
  matrix_2_1 (vec1 seven) (vec1 seven)

theorem left_sum_right_product_value :
  Equal (Vec (Vec Nat one) two)
    (matrix_product two three one (matrix_add two three b23 c23) factor_31)
    right_expected_21 = Refl

theorem matrix_product_distrib_right_closed :
  Equal (Vec (Vec Nat one) two)
    (matrix_product two three one (matrix_add two three b23 c23) factor_31)
    (matrix_add two one
      (matrix_product two three one b23 factor_31)
      (matrix_product two three one c23 factor_31)) = Refl
```

## stdlib/linear-algebra/matrix-product-distrib-left-refusal-same-input

The accepting control uses the square left factor below with the same
`b23` and `c23` as the refusal. The refusal input is made by taking a
fresh copy of the accepted client and appending only the false
right-hand side. No diagnostic code is specified by §4.3.

- spec: `spec/50-stdlib/65-linear-algebra.md` §4.3.
- promise class: durable invariant.
- given: `square_a22`, `b23`, and `c23`.
- expect: the correct `a(b+c) = ab+ac` control checks; the
  separately appended `a(b+c) = ab+c` claim is refused.
- measured: the accepting control and refusal share every operand;
  only the product `ac` is absent from the false right-hand side.
- why: an unrelated rejection cannot satisfy the refused claim.

```ken
const square_a22 : Vec (Vec Nat two) two =
  matrix_2_2 (vec2 one two) (vec2 zero one)

theorem same_input_correct_right_control :
  Equal (Vec (Vec Nat three) two)
    (matrix_product two two three square_a22 (matrix_add two three b23 c23))
    (matrix_add two three
      (matrix_product two two three square_a22 b23)
      (matrix_product two two three square_a22 c23)) = Refl
```

Append only this theorem to a fresh copy of the accepted client above:

```ken
theorem reject_dropped_right_product :
  Equal (Vec (Vec Nat three) two)
    (matrix_product two two three square_a22 (matrix_add two three b23 c23))
    (matrix_add two three
      (matrix_product two two three square_a22 b23)
      c23) = Refl
```

These closed observations complement, but do not prove, the four
generic §4 laws. No law over the vector argument of `matrix_vector`
is asserted; §4 explicitly leaves that orientation unspecified.
