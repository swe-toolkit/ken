# Right-cancellation of vector and matrix sums

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §20, approved child
`5f0fbf05eb83258d8a8c95244500717dc126b86d`.

This authored seed gives closed clients for §20. These finite clients
exercise law applications, not the generic proof bodies required by §20.
The public §20 exports are absent from the authoring base, so this seed
has not been checked or executed. No package-test, official
conformance-runner, CI, publication, landing, or trust result is claimed.

## Accepted client

The vector client uses `xs = ys = [1,2]` and `zs = [0,0]`. The matrix
client uses the same matrix on both sides and a nonzero common right
summand.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, matrix_add,
   vector_add_cancel_right, matrix_add_cancel_right)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1

fn vector2 (x : Nat) (y : Nat) : Vec Nat n2 =
  VCons Nat n1 x (VCons Nat n0 y (VNil Nat))

fn matrix2 (r0 : Vec Nat n2) (r1 : Vec Nat n2)
    : Vec (Vec Nat n2) n2 =
  VCons (Vec Nat n2) n1 r0
    (VCons (Vec Nat n2) n0 r1 (VNil (Vec Nat n2)))

const xs : Vec Nat n2 = vector2 n1 n2
const zs : Vec Nat n2 = vector2 n0 n0
const ys_reversed : Vec Nat n2 = vector2 n2 n1

const a22 : Vec (Vec Nat n2) n2 =
  matrix2 (vector2 n1 n0) (vector2 n2 n1)
const c22 : Vec (Vec Nat n2) n2 =
  matrix2 (vector2 n0 n1) (vector2 n1 n0)

theorem vector_cancel_equal
    : Equal (Vec Nat n2) xs xs =
  vector_add_cancel_right n2 xs xs zs Refl

theorem matrix_cancel_equal
    : Equal (Vec (Vec Nat n2) n2) a22 a22 =
  matrix_add_cancel_right n2 n2 a22 a22 c22 Refl
```

### stdlib/linear-algebra/vector-cancel-right-equal

- spec: §20 (`vector_add_cancel_right`).
- promise class: durable invariant.
- given: `xs = [1,2]` and `zs = [0,0]`.
- expect: the public vector law accepts with its sum-equality premise
  discharged by `Refl`, and concludes `xs = xs`.
- why: the law applies at a closed length-two vector and preserves its
  operand.

### stdlib/linear-algebra/matrix-cancel-right-equal

- spec: §20 (`matrix_add_cancel_right`).
- promise class: durable invariant.
- given: `a22` and `c22` above, each a `2 × 2` matrix.
- expect: the public matrix law accepts with its sum-equality premise
  discharged by `Refl`, and concludes `a22 = a22`.
- why: the law applies at a closed matrix shape with a shared, nonzero
  right summand.

## Isolated refused premise

Start from a fresh copy of the entire accepted client and append only
the theorem below. It keeps `xs` and `zs` fixed and changes only `ys`
from `[1,2]` to `[2,1]`. The two sums are therefore different. Refusal
must occur at the `Refl` supplied for the law's sole premise, before a
conclusion claim is reached; a later or unrelated refusal does not
satisfy the case.

### stdlib/linear-algebra/reject-vector-cancel-right-false-premise

- spec: §20 (`vector_add_cancel_right`).
- promise class: durable invariant.
- given: the accepted client, with `ys_reversed = [2,1]`.
- expect: reject at the `Refl` argument for the sum-equality premise.
- control: `vector_cancel_equal` accepts with the same `xs`, `zs`, law,
  and proof form when `ys = xs`.
- why: `[1,2] + [0,0]` differs from `[2,1] + [0,0]`, so the premise
  is false while the conclusion would claim `[1,2] = [2,1]`.

```ken
theorem vector_cancel_false_premise
    : Equal (Vec Nat n2) xs ys_reversed =
  vector_add_cancel_right n2 xs ys_reversed zs Refl
```

## Evidence boundary

The seed is an authored conformance case grounded in the approved §20
Spec child. Its expected outcomes have not been measured by a Ken check,
a package test, or an official conformance run.
