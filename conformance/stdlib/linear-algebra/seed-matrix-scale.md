# Scaling a matrix by a natural factor

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §16, in child
`018d11d7617c8ae7679c7a290723d4e8dcd2f2f9`.
Frame: Architect's AC-0 ruling `evt_3az75f678h2av`.

The accepted client checks the closed scaled matrix, then applies all eight
public laws to the ruled instances: `A = [[1,2,3],[4,5,6]]`,
`C = [[6,5,4],[3,2,1]]`, `B = [[1,2],[3,4],[5,6]]`, `x = 3`, and
`y = 2`. For each law, separate `Refl` declarations check its left and right
expressions against the same independently constructed concrete result; a
third declaration applies the generic public law. These closed rows complement,
but do not replace, the generic checked laws.

## Accepted client

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix
  (matrix_add,
    matrix_product,
    matrix_scale,
    matrix_zero,
    transpose,
    matrix_scale_distrib,
    matrix_scale_factor_distrib,
    matrix_scale_compose,
    matrix_scale_one,
    matrix_scale_zero,
    transpose_scale,
    matrix_product_scale,
    matrix_product_scale_right)

const zero : Nat = Zero

const one : Nat = Suc zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = add four four

const nine : Nat = add four five

const ten : Nat = add five five

const twelve : Nat = mul three four

const fifteen : Nat = mul three five

const eighteen : Nat = mul three six

const twenty : Nat = mul four five

const twenty_one : Nat = add twenty one

const twenty_four : Nat = mul six four

const twenty_five : Nat = mul five five

const thirty : Nat = mul five six

const thirty_six : Nat = mul six six

const sixty_six : Nat = add (mul six ten) six

const eighty_four : Nat = mul seven twelve

const one_hundred_forty_seven : Nat = add (mul seven twenty) seven

const one_hundred_ninety_two : Nat = mul eight twenty_four

fn vec2 (x : Nat) (y : Nat) : Vec Nat two = VCons Nat one x (VCons Nat zero y (VNil Nat))

fn vec3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat zero z (VNil Nat)))

fn matrix2x2 (r0 : Vec Nat two) (r1 : Vec Nat two) : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one r0 (VCons (Vec Nat two) zero r1 (VNil (Vec Nat two)))

fn matrix2x3 (r0 : Vec Nat three) (r1 : Vec Nat three) : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one r0 (VCons (Vec Nat three) zero r1 (VNil (Vec Nat three)))

fn matrix3x2
      (r0 : Vec Nat two) (r1 : Vec Nat two) (r2 : Vec Nat two)
    : Vec (Vec Nat two) three =
  VCons
    (Vec Nat two)
    two
    r0
    (VCons (Vec Nat two) one r1 (VCons (Vec Nat two) zero r2 (VNil (Vec Nat two))))

const a : Vec (Vec Nat three) two = matrix2x3 (vec3 one two three) (vec3 four five six)

const c : Vec (Vec Nat three) two = matrix2x3 (vec3 six five four) (vec3 three two one)

const b : Vec (Vec Nat two) three = matrix3x2 (vec2 one two) (vec2 three four) (vec2 five six)

const scaled_a : Vec (Vec Nat three) two =
  matrix2x3 (vec3 three six nine) (vec3 twelve fifteen eighteen)

const scaled_c : Vec (Vec Nat three) two =
  matrix2x3 (vec3 eighteen fifteen twelve) (vec3 nine six three)

const scale_sum : Vec (Vec Nat three) two =
  matrix2x3 (vec3 twenty_one twenty_one twenty_one) (vec3 twenty_one twenty_one twenty_one)

const scale_five_a : Vec (Vec Nat three) two =
  matrix2x3 (vec3 five ten fifteen) (vec3 twenty twenty_five thirty)

const scale_six_a : Vec (Vec Nat three) two =
  matrix2x3 (vec3 six twelve eighteen) (vec3 twenty_four thirty thirty_six)

const zero_a : Vec (Vec Nat three) two = matrix2x3 (vec3 zero zero zero) (vec3 zero zero zero)

const transpose_scaled_a : Vec (Vec Nat two) three =
  matrix3x2 (vec2 three twelve) (vec2 six fifteen) (vec2 nine eighteen)

const product_scaled : Vec (Vec Nat two) two =
  matrix2x2 (vec2 sixty_six eighty_four) (vec2 one_hundred_forty_seven one_hundred_ninety_two)

theorem scale_closed_value
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three three a) scaled_a =
  Refl

theorem scale_distrib_left_value
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three three (matrix_add two three a c))
        scale_sum =
  Refl

theorem scale_distrib_right_value
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_add two three (matrix_scale two three three a) (matrix_scale two three three c))
        scale_sum =
  Refl

theorem scale_distrib_application
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three three (matrix_add two three a c))
        (matrix_add
          two
          three
          (matrix_scale two three three a)
          (matrix_scale two three three c)) =
  matrix_scale_distrib two three three a c

theorem scale_factor_distrib_left_value
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three (add three two) a) scale_five_a =
  Refl

theorem scale_factor_distrib_right_value
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_add two three (matrix_scale two three three a) (matrix_scale two three two a))
        scale_five_a =
  Refl

theorem scale_factor_distrib_application
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three (add three two) a)
        (matrix_add two three (matrix_scale two three three a) (matrix_scale two three two a)) =
  matrix_scale_factor_distrib two three three two a

theorem scale_compose_left_value
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three (mul three two) a) scale_six_a =
  Refl

theorem scale_compose_right_value
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three three (matrix_scale two three two a))
        scale_six_a =
  Refl

theorem scale_compose_application
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three (mul three two) a)
        (matrix_scale two three three (matrix_scale two three two a)) =
  matrix_scale_compose two three three two a

theorem scale_one_left_value
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three one a) a =
  Refl

theorem scale_one_right_value : Equal (Vec (Vec Nat three) two) a a = Refl

theorem scale_one_application
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three one a) a =
  matrix_scale_one two three a

theorem scale_zero_left_value
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three zero a) zero_a =
  Refl

theorem scale_zero_right_value
    : Equal (Vec (Vec Nat three) two) (matrix_zero two three) zero_a =
  Refl

theorem scale_zero_application
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three zero a) (matrix_zero two three) =
  matrix_scale_zero two three a

theorem transpose_scale_left_value
    : Equal
        (Vec (Vec Nat two) three)
        (transpose two three (matrix_scale two three three a))
        transpose_scaled_a =
  Refl

theorem transpose_scale_right_value
    : Equal
        (Vec (Vec Nat two) three)
        (matrix_scale three two three (transpose two three a))
        transpose_scaled_a =
  Refl

theorem transpose_scale_application
    : Equal
        (Vec (Vec Nat two) three)
        (transpose two three (matrix_scale two three three a))
        (matrix_scale three two three (transpose two three a)) =
  transpose_scale two three three a

theorem product_scale_left_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two (matrix_scale two three three a) b)
        product_scaled =
  Refl

theorem product_scale_right_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_scale two two three (matrix_product two three two a b))
        product_scaled =
  Refl

theorem product_scale_application
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two (matrix_scale two three three a) b)
        (matrix_scale two two three (matrix_product two three two a b)) =
  matrix_product_scale two three two three a b

theorem product_scale_right_left_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two a (matrix_scale three two three b))
        product_scaled =
  Refl

theorem product_scale_right_right_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_scale two two three (matrix_product two three two a b))
        product_scaled =
  Refl

theorem product_scale_right_application
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two a (matrix_scale three two three b))
        (matrix_scale two two three (matrix_product two three two a b)) =
  matrix_product_scale_right two three two three a b

theorem scale_same_form_control
    : Equal (Vec (Vec Nat three) two) (matrix_scale two three three a) scaled_a =
  Refl
```

### `stdlib/linear-algebra/matrix-scale-closed-row`

- spec: §16.1.
- promise class: durable invariant.
- given: the nonzero `2 × 3` matrix `a` and factor `three`.
- expect: scaling each entry by `three` yields `[[3,6,9],[12,15,18]]`.
- why: the explicit expected matrix is built from vectors and constructors,
  not from `matrix_scale`.

### `stdlib/linear-algebra/matrix-scale-eight-laws-closed`

- spec: §16.1.
- promise class: durable invariant.
- given: `a`, `c`, `b`, `three`, and `two` in the accepted client.
- expect: each of the eight public laws applies, and each of its two sides
  independently reduces by `Refl` to the same explicit matrix.
- why: the rows instantiate the law interfaces at the ruled rectangular
  matrices and unequal factors, while independent closed values expose the
  concrete results.

## Isolated refused claims

For each refusal, start from a fresh copy of the complete accepted client
above and append only its theorem. The accepted `scale_same_form_control`
uses the same left endpoint and `Refl` proof form. Require a `TypeMismatch`
at the appended theorem's own `Refl`; do not pin a reason string.

### `stdlib/linear-algebra/refuse-scale-first-row-only`

- spec: §16.1–§16.2.
- promise class: durable invariant.
- given: `a` and factor `three` from the accepted client.
- expect: reject the claim that scaling changes only the first row.
- control: `scale_same_form_control` proves the same left endpoint equals
  `scaled_a` with `Refl`.
- why: the second row of the specified result is `[12,15,18]`, not
  `[4,5,6]`.

Append only this theorem:

```ken
theorem refuse_first_row_only
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three three a)
        (matrix2x3 (vec3 three six nine) (vec3 four five six)) =
  Refl
```

### `stdlib/linear-algebra/refuse-scale-adds-factor`

- spec: §16.1–§16.2.
- promise class: durable invariant.
- given: `a` and factor `three` from the accepted client.
- expect: reject the claim that scaling adds the factor to every entry.
- control: `scale_same_form_control` proves the same left endpoint equals
  `scaled_a` with `Refl`.
- why: the first entry is `3`, not `1 + 3 = 4`.

Append only this theorem:

```ken
theorem refuse_adds_factor
    : Equal
        (Vec (Vec Nat three) two)
        (matrix_scale two three three a)
        (matrix2x3 (vec3 four five six) (vec3 seven eight nine)) =
  Refl
```

## Evidence boundary

The seed is authored against Architect's AC-0 PASS on exact base
`38796115f264ac20d424f2a0119cb67abbdf55ce` (`evt_3az75f678h2av`). Its
exact §16 source is Spec child
`018d11d7617c8ae7679c7a290723d4e8dcd2f2f9`. The local named Rust probe on an
isolated D0 overlay accepted the complete client 1/1 and refused each twin at
the appended theorem's `Refl` (TypeMismatch), 2/2. This is scratch-overlay
seed checking, not official conformance execution. The D0's package checks and
mutant table are Architect-reported measurements, not execution of this seed or
of an official conformance runner. No CI, publication, landing, or landed trust
measurement is claimed.
