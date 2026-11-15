# Scaling a matrix before applying it to a vector

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §17, in Spec child
`0a53d561815770270f8c48022f014390add109e0`.
Frame: Architect's AC-0 ruling `evt_4mkask1atc4pj`.

The accepted client checks the scaled matrix application and scaling the
unscaled result independently against `[9, 21]`, then applies the public
law at the same instance. R-VAL (`scaled_value`), R-RIGHT (`right_value`),
and R-LAW (`law_at_value`) pin the discriminator in §17; these closed rows
do not establish the generic theorem for every dimension and value.

## Accepted client

Here `a = [[1, 2], [3, 4]]`, `x = [1, 1]`, and the factor is `3`.
The independently constructed scaled result is `[9, 21]`; the unscaled
application is `[3, 7]`.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix
  (matrix_vector, matrix_scale, vector_scale, matrix_vector_scale)

const zero : Nat = Zero

const one : Nat = Suc zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const seven : Nat = add three four

const nine : Nat = mul three three

const twenty_one : Nat = mul three seven

const a : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (VCons Nat one one (VCons Nat zero two (VNil Nat)))
    (VCons
      (Vec Nat two)
      zero
      (VCons Nat one three (VCons Nat zero four (VNil Nat)))
      (VNil (Vec Nat two)))

const x : Vec Nat two = VCons Nat one one (VCons Nat zero one (VNil Nat))

const scaled_result : Vec Nat two = VCons Nat one nine (VCons Nat zero twenty_one (VNil Nat))

const unscaled_result : Vec Nat two = VCons Nat one three (VCons Nat zero seven (VNil Nat))

theorem scaled_value
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_scale two two three a) x)
        scaled_result =
  Refl

theorem right_value
    : Equal (Vec Nat two) (vector_scale two three (matrix_vector two two a x)) scaled_result =
  Refl

theorem law_at_value
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_scale two two three a) x)
        (vector_scale two three (matrix_vector two two a x)) =
  matrix_vector_scale two two three a x
```

### `stdlib/linear-algebra/matrix-vector-scale-closed-values`

- spec: §17.
- promise class: durable invariant.
- given: `a`, `x`, and factor `three` from the accepted client.
- expect: `scaled_value` and `right_value` both accept with the independent
  result `[9, 21]`.
- why: scaling the matrix before applying it and scaling the unscaled
  application must produce the same indexed vector.

### `stdlib/linear-algebra/matrix-vector-scale-public-law-application`

- spec: §17.
- promise class: durable invariant.
- given: `a`, `x`, and factor `three` from the accepted client.
- expect: the public `matrix_vector_scale` theorem applies to this instance.
- why: the concrete result checks and the public theorem application at this
  instance are distinct observations; neither substitutes for the other.

## Isolated refused claim

Run the refused theorem in a fresh copy of the complete accepted client above
and append only this theorem. `scaled_value` is an accepted control with the
same left endpoint and `Refl` form. Require refusal at the appended theorem's
own `Refl`; do not pin diagnostic wording.

### `stdlib/linear-algebra/refuse-matrix-vector-scale-unscaled-result`

- spec: §17.
- promise class: durable invariant.
- given: `a`, `x`, and factor `three` from the accepted client.
- expect: refuse the claim that scaling the matrix before application yields
  the unscaled result `[3, 7]`.
- why: the scaled application is `[9, 21]`; `[3, 7]` is the unscaled
  application. The accepted same-form control keeps the refusal from passing
  on an import, premise, or unrelated earlier error.

Append only X-UNSCALED, this theorem:

```ken
theorem unscaled_value
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_scale two two three a) x)
        unscaled_result =
  Refl
```

## Evidence boundary

Architect's AC-0 D0 on landed base `cb25f536df8e4f5a0ec6e75d33de6086ad2088b3`
(`evt_4mkask1atc4pj`) checked the generic law and an importing client. Local
checks for this seed used a detached `cb25f536` overlay with the exact Spec
child `0a53d561815770270f8c48022f014390add109e0` and Foundation-child catalog
source `cd4f49a8f10a4bd8adda7d64ac672c4dfdb0ea32`. The catalog package check,
accepted-client format and check, and full-prefix refusal format passed. The
full-prefix refusal check returned E0038 `TypeMismatch` at the appended
`Refl` (byte span `1623..1627`). The prebuilt CLI was from base
`273981a28868648994e656873dac25180a524d0b`; its CLI, elaborator, kernel and
diagnostic production sources are unchanged through `cb25f536`. This is local
seed validation, not official conformance execution, CI, publication, landing,
or a landed-trust measurement.
