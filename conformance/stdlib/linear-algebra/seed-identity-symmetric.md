# The identity matrix is its own transpose

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §8.
Rulings: Architect AC-0 `evt_4qsf0eppzbc6q`; Spec Leader release
`evt_5c933mt43r0gg`.

The accepted client applies the public `transpose_identity` law at
`n = 3`. Separate `Refl` controls compare `identity three` and its
transpose with the same explicitly constructed 3 × 3 identity. The
closed values check the fixture; they do not prove the generic law.

## Accepted shared client

The explicit 3 × 3 identity fixes its diagonal and all six off-diagonal positions.
The law application is separate from the two closed value controls.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (identity, transpose, transpose_identity)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

fn vec2 (p : Nat) (q : Nat) : Vec Nat two = VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

const identity3 : Vec (Vec Nat three) three =
  VCons
    (Vec Nat three)
    two
    (vec3 one Zero Zero)
    (VCons
      (Vec Nat three)
      one
      (vec3 Zero one Zero)
      (VCons (Vec Nat three) Zero (vec3 Zero Zero one) (VNil (Vec Nat three))))

const upper : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one two)
    (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two)))

const upper_transposed : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one Zero)
    (VCons (Vec Nat two) Zero (vec2 two one) (VNil (Vec Nat two)))

theorem identity_three_value : Equal (Vec (Vec Nat three) three) (identity three) identity3 =
  Refl

theorem transpose_identity_three_value
    : Equal (Vec (Vec Nat three) three) (transpose three three (identity three)) identity3 =
  Refl

theorem transpose_identity_three_law
    : Equal
        (Vec (Vec Nat three) three)
        (transpose three three (identity three))
        (identity three) =
  transpose_identity three

theorem upper_transpose_value
    : Equal (Vec (Vec Nat two) two) (transpose two two upper) upper_transposed =
  Refl

theorem upper_same_form_control : Equal (Vec (Vec Nat two) two) upper upper = Refl
```

### stdlib/linear-algebra/transpose-identity-three

- spec: §8.1–§8.2.
- promise class: durable invariant.
- given: the public law at `three`, plus `identity3` with rows
  `[1,0,0]`, `[0,1,0]`, and `[0,0,1]`.
- expect: both closed value controls check with `Refl`, and the
  theorem application checks with `transpose_identity three`.
- measured: the diagonal and all six off-diagonal entries in the
  constructor-built matrix; the law is independently applied.
- claimed: transposing the constructed identity preserves its
  indexed entries at dimension three.
- the gap: these finite values do not prove the law for every `n`;
  §8.1 states that generic law.
- why: identity and transpose are each compared to an independent
  literal, not merely to each other.

## Isolated refused claim

Append only `seed_neg` to a fresh complete copy of the accepted client.
The transpose oracle establishes rows `[1,0]` and `[2,1]`, which
are not the original rows `[1,2]` and `[0,1]`. The accepted
`upper_same_form_control` uses the same `Equal` and `Refl` form.
Require refusal at the appended claim's own `Refl`.

### stdlib/linear-algebra/reject-symmetry-of-upper-triangular

- spec: §8.2.
- promise class: durable invariant.
- given: the accepted client and its `upper_transpose_value` oracle.
- expect: refuse the false equality between `transpose upper` and
  `upper` at its `Refl`.
- measured: the same 2 × 2 carrier and proof form as the accepted
  transpose oracle and reflexive control.
- claimed: a general square matrix need not equal its transpose.
- the gap: this one matrix refutes symmetry as a general law; it
  does not weaken the identity-specific theorem.
- why: the upper entry `2` moves to the lower position under
  transpose, and the false claim is isolated after an accepted prefix.

```ken
theorem seed_neg : Equal (Vec (Vec Nat two) two) (transpose two two upper) upper = Refl
```

## Evidence boundary

On an isolated D0 overlay matching Architect's patch SHA-256
`74cff3e2237167533ec8324c0193ab140b871662e3cd7cf394635f46f0a1db4a`,
the accepted client passed `ken check` (rc 0). A fresh-prefix client
with only `seed_neg` appended was refused at that claim's own `Refl`
(span 1734–1738, `TypeMismatch: Refl expects an Eq-shaped goal`).
`ken fmt --check` passed for both extracted source clients. The overlay
and scratch clients were removed before commit. The seed-only candidate
does not yet contain the public catalog export; Foundation owns that
following turn. These checks are candidate-local, not an official
conformance-runner result. No official conformance execution, CI,
publication, or landing is claimed.
