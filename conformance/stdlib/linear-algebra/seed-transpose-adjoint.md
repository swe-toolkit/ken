# Transpose is adjoint to matrix-vector application

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §7.
Rulings: Architect AC-0 `evt_5yxmg6w6x2cp1`; Spec Leader release
`evt_7pdvep0mp02p7`.

The accepted client applies the premise-free public law on a non-square
`2 × 3` matrix and independently computes both natural dot values. It
also pins the transposed matrix as an indexed `Vec` equality and the
right side with one swapped input to a distinct value. These closed
calculations do not prove the generic law or cover all zero dimensions.
The §7 catalog export is absent on the authoring base; the code fences
are conformance obligations, not shipped-export results.

## Accepted shared client

Let `a` have rows `[1,2,3]` and `[4,0,1]`, `x = [2,1,3]`, and
`y = [1,2]`. Matrix-vector application gives `[13,11]` and a dot
result of `35`. Transposition gives `[[1,4],[2,0],[3,1]]`; applying
this to `y` gives `[9,2,5]`, whose dot with `x` is also `35`.
Replacing `y` with `[2,1]` only on the transposed side instead gives
`[6,4,7]` and a dot result of `37`.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (dot, matrix_vector, transpose, transpose_adjoint)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const seven : Nat = Suc (Suc five)

const nine : Nat = Suc (Suc seven)

const eleven : Nat = Suc (Suc nine)

const thirteen : Nat = Suc (Suc eleven)

const thirty_five : Nat = mul five seven

const thirty_seven : Nat = add thirty_five two

fn vec2 (p : Nat) (q : Nat) : Vec Nat two = VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three = VCons Nat two p (vec2 q r)

const a : Vec (Vec Nat three) two =
  VCons
    (Vec Nat three)
    one
    (vec3 one two three)
    (VCons (Vec Nat three) Zero (vec3 four Zero one) (VNil (Vec Nat three)))

const x : Vec Nat three = vec3 two one three

const y : Vec Nat two = vec2 one two

const y_swapped : Vec Nat two = vec2 two one

const transposed : Vec (Vec Nat two) three =
  VCons
    (Vec Nat two)
    two
    (vec2 one four)
    (VCons
      (Vec Nat two)
      one
      (vec2 two Zero)
      (VCons (Vec Nat two) Zero (vec2 three one) (VNil (Vec Nat two))))

theorem transpose_value : Equal (Vec (Vec Nat two) three) (transpose two three a) transposed =
  Refl

theorem left_image_value
    : Equal (Vec Nat two) (matrix_vector two three a x) (vec2 thirteen eleven) =
  Refl

theorem right_image_value
    : Equal
        (Vec Nat three)
        (matrix_vector three two (transpose two three a) y)
        (vec3 nine two five) =
  Refl

theorem swapped_image_value
    : Equal
        (Vec Nat three)
        (matrix_vector three two (transpose two three a) y_swapped)
        (vec3 (Suc five) four seven) =
  Refl

theorem left_value : Equal Nat (dot two (matrix_vector two three a x) y) thirty_five = Proved

theorem right_value
    : Equal Nat (dot three x (matrix_vector three two (transpose two three a) y)) thirty_five =
  Proved

theorem swapped_right_value
    : Equal Nat
        (dot three x (matrix_vector three two (transpose two three a) y_swapped))
        thirty_seven =
  Proved

theorem adjoint_closed
    : Equal Nat
        (dot two (matrix_vector two three a x) y)
        (dot three x (matrix_vector three two (transpose two three a) y)) =
  transpose_adjoint two three a x y

theorem adjoint_closed_value
    : Equal Nat
        (dot two (matrix_vector two three a x) y)
        (dot three x (matrix_vector three two (transpose two three a) y)) =
  Proved
```

### stdlib/linear-algebra/transpose-adjoint-nonsquare

- spec: §7.1.
- promise class: durable invariant.
- given: `a`, `x`, and `y` as above.
- expect: the public law applies, and each side independently checks as
  35 by `Proved`; the transpose and intermediate images have their
  separately constructed structural `Vec` oracles.
- why: closed agreement is evidence for this fixture, not for the
  universal law or zero-dimensional cases.

## Isolated refused claim

Start from a **fresh complete copy** of the accepted client and append
only the claim below. The left endpoint and `x` and `a` are unchanged;
only the right endpoint receives `y_swapped`. The accepted prefix
checks the left and original right as 35 and the substituted right as
37 independently. `adjoint_closed_value` uses the same `Equal Nat` and
`Proved` form with the correct right endpoint. Require refusal at the
appended `Proved`, not at an earlier import, computation, or law
application.

### stdlib/linear-algebra/reject-adjoint-with-swapped-right-y

- spec: §7.1.
- promise class: durable invariant.
- given: the exact accepted prefix and its `y_swapped` value control.
- expect: refuse the false equality `35 = 37`.
- why: adjointness keeps the **same** `y` on both sides; exchanging only
  the right operand changes the result.

```ken
theorem seed_neg
    : Equal Nat
        (dot two (matrix_vector two three a x) y)
        (dot three x (matrix_vector three two (transpose two three a) y_swapped)) =
  Proved
```

## Evidence boundary

Architect's AC-0 on detached `54f68d3cc` checked the generic proof in
a scratch catalog patch with no new `Axiom`, `ken check` and
`ken fmt --check` rc 0, and an unchanged scratch trusted GlobalId set
109 → 109. Its baseline client checked the public law and independent
35-value `Proved` controls. Its two swapped-`y` variants were refused
(`KernelRejected TypeMismatch`), including the same `Proved` form
chosen above. The D0 patch SHA-256 is
`aadeb73d1e6661a39c737b66369bc26c9050c689a5a1291db2451475092219ab`.
The author checked this seed on a disposable `54f68d3cc` worktree
with only the NatMatrix catalog hunk of that exact D0 patch overlaid.
The full accepted prefix checked rc 0; after formatting, both the
accepted client and fresh-prefix refused client passed
`ken fmt --check` rc 0. Appending only `seed_neg` failed rc 1/K0004
`KernelRejected TypeMismatch` at bytes 2604–2779, inside the appended
theorem; its `Proved` begins at byte 2773. On untouched main, the
independent value controls and accepted same-form closed equality
checked rc 0 without the unavailable law, while the full client
failed rc 1/E0012 `UnboundName` at the missing `transpose_adjoint`
import. Logs under `/workspaces/ken/local/` use prefix
`spec-author-la-adjoint-` and suffixes `overlay-positive-canonical-check`,
`overlay-negative-canonical-check`, `overlay-positive-fmt`,
`overlay-negative-fmt`, `base-values`, and `base-law` (`.log`). The
reused CLI was built by the Architect's D0, not independently built
from the detached base. Scratch worktrees were removed. These
candidate-local checks and the D0 do not establish a shipped §7
export, official conformance-runner execution, CI, publication,
landing, or landed trust.
