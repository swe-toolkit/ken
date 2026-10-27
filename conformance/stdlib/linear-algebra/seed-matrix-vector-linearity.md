# Matrix-vector linearity in the vector argument

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §6.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY.md`.

The accepted client applies both public laws to a non-square `2 × 3`
matrix and distinct length-three vectors. Four independently written
`Refl` oracles check the separate images of `x`, `y`, `x + y`, and
`3x` against concrete indexed vectors. The last `Refl` control uses
the same left endpoint and proof form as the refused claim below,
but has the correct right endpoint. These closed examples do not
prove either generic law. The two §6 exports have checked bodies in the
catalog candidate, and a root-loaded package test checks this accepted
client and the isolated refusal. D0-overlay checks remain historical; the
package test is not official conformance-runner execution, CI, publication,
landing or landed
trust. Each case promises a durable invariant.

## Accepted shared client

Here `a23 = [[1,0,2],[0,1,1]]`, `x3 = [1,2,0]` and
`y3 = [0,1,3]`. Independently computed images are `a23 x3 = [1,2]`,
`a23 y3 = [6,4]`, `a23 (x3 + y3) = [7,6]`, and
`a23 (3x3) = [3,6]`. The accepted client must check in full before
any negative claim is appended.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, matrix_vector, matrix_vector_additive, matrix_vector_homogeneous)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six

fn vec2 (p : Nat) (q : Nat) : Vec Nat two = VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

const a23 : Vec (Vec Nat three) two =
  VCons
    (Vec Nat three)
    one
    (vec3 one Zero two)
    (VCons (Vec Nat three) Zero (vec3 Zero one one) (VNil (Vec Nat three)))

const x3 : Vec Nat three = vec3 one two Zero

const y3 : Vec Nat three = vec3 Zero one three

theorem image_x_value : Equal (Vec Nat two) (matrix_vector two three a23 x3) (vec2 one two) = Refl

theorem image_y_value : Equal (Vec Nat two) (matrix_vector two three a23 y3) (vec2 six four) = Refl

theorem image_sum_value
    : Equal (Vec Nat two) (matrix_vector two three a23 (vector_add three x3 y3)) (vec2 seven six) =
  Refl

theorem image_scaled_value
    : Equal (Vec Nat two) (matrix_vector two three a23 (vector_scale three three x3)) (vec2 three six) =
  Refl

theorem additive_closed
    : Equal
        (Vec Nat two)
        (matrix_vector two three a23 (vector_add three x3 y3))
        (vector_add two (matrix_vector two three a23 x3) (matrix_vector two three a23 y3)) =
  matrix_vector_additive two three a23 x3 y3

theorem homogeneous_closed
    : Equal
        (Vec Nat two)
        (matrix_vector two three a23 (vector_scale three three x3))
        (vector_scale two three (matrix_vector two three a23 x3)) =
  matrix_vector_homogeneous two three three a23 x3

theorem additive_closed_refl
    : Equal
        (Vec Nat two)
        (matrix_vector two three a23 (vector_add three x3 y3))
        (vector_add two (matrix_vector two three a23 x3) (matrix_vector two three a23 y3)) =
  Refl
```

### stdlib/linear-algebra/matrix-vector-additive-closed

- spec: §6.1.
- promise class: durable invariant.
- given: `a23`, `x3`, and distinct `y3` above.
- expect: the `2 × 3` additive law applies; separate `Refl` oracles
  compute the two original images and the image of their sum.
- why: the §4.2 matrix-sum orientation does not assert this law about
  the vector argument.

### stdlib/linear-algebra/matrix-vector-homogeneous-closed

- spec: §6.1.
- promise class: durable invariant.
- given: the same non-square matrix, `x3`, and factor `three`.
- expect: the homogeneous law applies and an independent `Refl`
  oracle computes the scaled image `[3,6]`.
- why: scalar multiplication on the vector before mapping must
  agree with scaling the length-two result afterward.

## Isolated refused claim

Start from a fresh copy of the **entire accepted client** and append
only the theorem below. Its left endpoint is exactly the one in
`image_sum_value` and `additive_closed_refl`; both accepted controls
use `Refl` with a true right endpoint. The new theorem asserts the
false equality `[7,6] = [1,2]`, not a second proof form or a broken
fixture. Require a refusal at the appended theorem, not at an earlier
import, value oracle or law application.

### stdlib/linear-algebra/reject-sum-image-equals-x-image

- spec: §6.1.
- promise class: durable invariant.
- given: `a23`, `x3`, and `y3` from the accepted prefix.
- expect: refuse the false claim that mapping `x3 + y3` equals
  mapping `x3` alone.
- why: the image of `y3` is `[6,4]`, not zero, and additivity
  cannot discard it.

```ken
theorem seed_neg
    : Equal (Vec Nat two) (matrix_vector two three a23 (vector_add three x3 y3)) (matrix_vector two three a23 x3) =
  Refl
```

## Evidence boundary

Architect's detached AC-0 D0 on `aa6370449` checked the two generic
proofs and this exact closed client as a scratch elaborate-file
client. The full positive block was accepted. Appending only
`seed_neg` to that accepted block returned `TypeMismatch` at its
`Refl` because `[7,6]` differs from `[1,2]`. Its independent
`image_sum_value` and `additive_closed_refl` controls had the same
left endpoint and proof form. The D0 patch digest is
`fb38570afef9c3d16e26aa9acdee937067ffe1a84f73eff9512255da829fc390`.
The author rechecked the exact positive source and a fresh complete
positive prefix followed by only `seed_neg` on a detached `aa6370449`
worktree with that D0 patch. A preexisting Ken CLI accepted the positive
client (rc 0); the false client returned rc 1/E0038 `TypeMismatch`,
`Refl expects an Eq-shaped goal`, at bytes 2204–2208 inside the
appended theorem. On the unpatched base, the four value oracles with
only their required imports accepted (rc 0), while the complete
positive client failed at the missing public
`matrix_vector_additive` import (rc 1/E0012 `UnboundName`). Logs
are in `/workspaces/ken/local/` under `spec-author-mv-` with suffixes
`positive`, `false`, `base-value-controls`, and `base-laws` (`.log`).
The scratch overlay and worktree were removed; the CLI binary predates
the detached base and is not an exact-base toolchain measurement.
These D0 observations test proposed scratch proofs and closed instances,
not §6 exports on main. The candidate package test checks the accepted
client and isolated refusal against root-loaded §6 exports, but neither
its result nor this seed text establishes official conformance-runner
execution, CI, publication, landing or landed trust.
