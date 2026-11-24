# Orientation is unchanged by a shared translation

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §21, child
`8cf28f9f04935bccdf7a6b794aab7c5c464e06a0`.

This seed exercises the public shared-translation law on one closed
counter-clockwise triple. It also separates that case from moving only
one point. The finite applications and controls do not establish the
generic proof body. This seed has not been run by the official
conformance runner. No CI, publication, landing, or trust measurement
is claimed.

## Accepted shared translation

The original triple `(0,0), (1,0), (0,1)` has turn `Gt`. Translating
all three points by `(2,3)` gives `(2,3), (3,3), (2,4)`. Its cross
sums are `L = 24` and `R = 23`, so its turn is also `Gt`.

```ken
import Algorithm.Geometry.NatBox (Point, MkPoint)

import Algorithm.Geometry.NatOrientation (orient, orient_translate)

import Core.Logic.OrdResult (OrdResult, Gt, Lt)

const n0 : Nat = Zero

const n1 : Nat = Suc n0

const n2 : Nat = Suc n1

const n3 : Nat = Suc n2

const n4 : Nat = Suc n3

const point_00 : Point = MkPoint n0 n0

const point_10 : Point = MkPoint n1 n0

const point_01 : Point = MkPoint n0 n1

const point_23 : Point = MkPoint n2 n3

const point_33 : Point = MkPoint n3 n3

const point_24 : Point = MkPoint n2 n4

theorem original_turn_is_gt : Equal OrdResult (orient point_00 point_10 point_01) Gt = Proved

theorem translated_turn_is_gt : Equal OrdResult (orient point_23 point_33 point_24) Gt = Proved

theorem shared_translation_by_law
    : Equal OrdResult (orient point_23 point_33 point_24) (orient point_00 point_10 point_01) =
  orient_translate n0 n0 n1 n0 n0 n1 n2 n3

theorem shared_translation_by_proved
    : Equal OrdResult (orient point_23 point_33 point_24) (orient point_00 point_10 point_01) =
  Proved
```

### `stdlib/geometry/orient-translate-shared`

- spec: §21.
- promise class: durable invariant.
- given: the triple `(0,0), (1,0), (0,1)` and its translation by
  `(2,3)`.
- expect: `orient_translate` accepts and concludes that the moved
  triple has the same turn as the original; both direct controls give
  `Gt`.
- measured: all three points receive the same vector; the moved
  triple's cross sums are `24` and `23`.
- claimed: adding one natural vector to all three points preserves
  their `orient` result.
- gap: this closed application does not establish the generic law.
- why: the public law application and direct value controls exercise
  the shared translation at one nonzero vector.

## Refusal when only one point moves

Start from a fresh copy of the complete accepted client above, then
append the `Lt` control and refusal below. The final theorem is the
same equality form as the accepted direct control. The first point is
`(2,3)` in both rows; only the other two points differ. The
one-point-shifted triple has `L = 1`, `R = 5`, and turn `Lt`, unlike
the original `Gt`.

### `stdlib/geometry/orient-translate-one-point-refusal`

- spec: §18.1 and §21.
- promise class: durable invariant.
- given: the accepted client, then the direct `Lt` control and the
  same-form equality claim for `(2,3), (1,0), (0,1)` versus the
  original triple.
- expect: the `Lt` control checks; the final equality claim refuses
  with `KernelRejected(TypeMismatch)` at its own declaration. Do not
  count a refusal in the accepted prefix or control as success.
- measured: the one-point-shifted cross sums are `1` and `5`; the
  accepted prefix establishes the original turn as `Gt`.
- claimed: the shared-translation law does not make a single-point
  displacement preserve orientation.
- gap: this finite counterexample does not establish the generic law
  or a diagnostic span or wording.
- why: moving only the first point changes the turn from `Gt` to `Lt`.

```ken
theorem one_point_shift_turn_is_lt : Equal OrdResult (orient point_23 point_10 point_01) Lt =
  Proved

theorem refuse_one_point_shift_as_shared_translation
    : Equal OrdResult (orient point_23 point_10 point_01) (orient point_00 point_10 point_01) =
  Proved
```

The accepted and refused rows distinguish a shared vector from a
one-point displacement; neither row tests signed coordinates,
point-rotation, or a generic proof body.
