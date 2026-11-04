# The zero matrix absorbs matrix products

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §13, as ruled by the
final Architect AC-0 for SPEC-LINEAR-ALGEBRA-MATRIX-PRODUCT-ZERO.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-MATRIX-PRODUCT-ZERO.md`.

Status: authored on the prestated seed child. The six accepted law
applications and the isolated Refl refusal/control pair follow final
AC-0. They are not a substitute for the generic checked laws. Foundation
QA reports that, at candidate
`512b3d1e25ea0043877cb375450fa5243b42dc29`, a temporary roots-loaded
harness extracted the exact seed fences. The original import failed at
`transpose`; adding only the missing import made the accepted and
refused fences pass the temporary harness 1/1. This is not official
conformance-runner execution. Foundation's and Architect's scratch D0
results are attributed; CV ran no package tests or official runner.

## Accepted shared client

The rows of `a` are `[1,2]` and `[2,1]`. `empty` has zero rows and
two columns. The applications below use the public law types fixed by
final AC-0: the left and right product laws take `(m) (n) (p)` and
their free factor; `transpose_zero` takes `(m) (n)`.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix
  (matrix_product,
    matrix_zero,
    identity,
    matrix_product_zero_left,
    matrix_product_zero_right,
    transpose,
    transpose_zero)

const one : Nat = Suc Zero

const two : Nat = Suc one

fn pair (x : Nat) (y : Nat) : Vec Nat two = VCons Nat one x (VCons Nat Zero y (VNil Nat))

fn rows2 (r0 : Vec Nat two) (r1 : Vec Nat two) : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one r0 (VCons (Vec Nat two) Zero r1 (VNil (Vec Nat two)))

const a : Vec (Vec Nat two) two = rows2 (pair one two) (pair two one)

const empty : Vec (Vec Nat two) Zero = VNil (Vec Nat two)

theorem matrix_product_zero_left_nonzero
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two (matrix_zero two two) a)
        (matrix_zero two two) =
  matrix_product_zero_left two two two a

theorem matrix_product_zero_right_nonzero
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two a (matrix_zero two two))
        (matrix_zero two two) =
  matrix_product_zero_right two two two a

theorem matrix_product_zero_left_empty
    : Equal
        (Vec (Vec Nat two) Zero)
        (matrix_product Zero two two (matrix_zero Zero two) a)
        (matrix_zero Zero two) =
  matrix_product_zero_left Zero two two a

theorem matrix_product_zero_right_empty
    : Equal
        (Vec (Vec Nat two) Zero)
        (matrix_product Zero two two empty (matrix_zero two two))
        (matrix_zero Zero two) =
  matrix_product_zero_right Zero two two empty

theorem transpose_zero_square
    : Equal
        (Vec (Vec Nat two) two)
        (transpose two two (matrix_zero two two))
        (matrix_zero two two) =
  transpose_zero two two

theorem transpose_zero_zero_rows
    : Equal
        (Vec (Vec Nat Zero) two)
        (transpose Zero two (matrix_zero Zero two))
        (matrix_zero two Zero) =
  transpose_zero Zero two

theorem matrix_product_zero_right_same_form_control
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two a (matrix_zero two two))
        (matrix_zero two two) =
  Refl
```

### stdlib/linear-algebra/matrix-product-zero-laws-closed

- spec: §13.
- promise class: durable invariant.
- given: the nonzero `2 × 2` matrix `a` and the zero-row matrix `empty`.
- expect: each product law applies to `a`, and each applies at zero
  rows; `transpose_zero` applies at `(two, two)` and `(Zero, two)`.
- why: the six applications cover both product orientations on a
  nonzero matrix and at zero rows, plus both ruled transpose cases.

## Isolated refused claim

Start from a fresh copy of the entire accepted client and append only the
false theorem below. Its product expression, result type, and `Refl`
form match the accepted control. Only the right factor changes. With
`a = [[1,2],[2,1]]`, §1.3's right-identity law gives `a` as the
product with `identity two`. Since `a` is nonzero, the false claim must
be refused at its own proof span.

### stdlib/linear-algebra/reject-identity-factor-as-zero-product

- spec: §13.
- promise class: durable invariant.
- given: the accepted client with `a = [[1,2],[2,1]]`.
- expect: refuse the identity-factor claim at its `Refl` proof and
  accept the same-form zero-factor control.
- control: the accepted client proves the product with right factor
  `matrix_zero two two` equals `matrix_zero two two` using `Refl`.
- why: the factor is the isolated difference; the product form and
  proof form remain fixed. Architect's final AC-0 records that `Refl`
  refuses the identity-factor row and accepts the zero-factor control;
  `Proved` refuses both and therefore does not discriminate.

Append only this theorem to a fresh copy of the accepted client:

```ken
theorem reject_identity_as_zero_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two a (identity two))
        (matrix_zero two two) =
  Refl
```


## Evidence and originality boundary

Final AC-0's scratch measurements are Architect-reported D0 evidence.
They report the identity-factor `Refl` row refused and the zero-factor
`Refl` control accepted. Foundation's implementer ran the D0 probe at
base `165a98acec79d5e367bfeaa5472e0fd9149313fb`; Foundation reported
it passed 2/2 with equal trusted identity sets (109 → 109) and no added
`Axiom`. Foundation QA separately reports that the unmodified exact
candidate seed failed on the missing `transpose` import, and that its
import-only temporary control passed the accepted and refused fences
1/1. That is QA-reported scratch evidence, not an official conformance
run; CV ran no package tests or official runner.

No `local/refs` or external reference source was consulted. This seed
is based on the Ken spec, the frame, and the final AC-0 ruling.
