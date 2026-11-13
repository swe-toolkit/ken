# Natural-point orientation

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §18.
Frame: `docs/program/issues/SPEC-GEOMETRY-NAT-ORIENTATION.md`.

These cases exercise the exact natural-point orientation and four public
laws in §18.1–§18.2. The closed outputs are stated with `Proved`; separate
closed clients apply each public law. The cases check the interface and
selected values, not the generic proof bodies.

## Accepted client

The three triples have turns `Gt`, `Lt`, and `Eq`, respectively. The
collinear triple has no zero coordinate. Each public law is used at a
closed input, and its resulting proposition is also checked in the
measured `Proved` form.

```ken
import Algorithm.Geometry.NatOrientation
  (orient, orient_swap, orient_rotation, orient_repeated_point, orient_cross)
import Algorithm.Geometry.NatBox (Point, MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_result_flip)
import Data.Numeric.Nat.Order (compare)
import Data.Numeric.Nat.Arithmetic (add, mul)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const n6 : Nat = Suc n5
const n7 : Nat = Suc n6
const n8 : Nat = Suc n7
const n9 : Nat = Suc n8
const n10 : Nat = Suc n9
const n11 : Nat = Suc n10

const p00 : Point = MkPoint n0 n0
const p10 : Point = MkPoint n1 n0
const p01 : Point = MkPoint n0 n1
const p11 : Point = MkPoint n1 n1
const p22 : Point = MkPoint n2 n2
const p33 : Point = MkPoint n3 n3

theorem ccw_left_is_one
  : Equal Nat (add (add (mul n0 n0) (mul n1 n1)) (mul n0 n0)) n1 = Proved

theorem ccw_right_is_zero
  : Equal Nat (add (add (mul n0 n1) (mul n0 n0)) (mul n1 n0)) n0 = Proved

theorem ccw_is_gt : Equal OrdResult (orient p00 p10 p01) Gt = Proved

theorem cw_is_lt : Equal OrdResult (orient p10 p00 p01) Lt = Proved

theorem collinear_left_is_eleven
  : Equal Nat (add (add (mul n1 n2) (mul n2 n3)) (mul n3 n1)) n11 = Proved

theorem collinear_right_is_eleven
  : Equal Nat (add (add (mul n1 n2) (mul n2 n3)) (mul n3 n1)) n11 = Proved

theorem collinear_is_eq : Equal OrdResult (orient p11 p22 p33) Eq = Proved

theorem closed_swap_is_proved
  : Equal OrdResult (orient p10 p00 p01)
      (ord_result_flip (orient p00 p10 p01)) = Proved

theorem closed_swap_law_application
  : Equal OrdResult (orient p10 p00 p01)
      (ord_result_flip (orient p00 p10 p01)) =
    orient_swap p00 p10 p01

theorem closed_rotation_is_proved
  : Equal OrdResult (orient p00 p10 p01) (orient p10 p01 p00) = Proved

theorem closed_rotation_law_application
  : Equal OrdResult (orient p00 p10 p01) (orient p10 p01 p00) =
    orient_rotation p00 p10 p01

theorem closed_repeated_point_is_proved
  : Equal OrdResult (orient p00 p00 p01) Eq = Proved

theorem closed_repeated_point_law_application
  : Equal OrdResult (orient p00 p00 p01) Eq =
    orient_repeated_point p00 p01

theorem closed_cross_is_proved
  : Equal OrdResult (orient p00 p10 p01)
      (compare
        (add (add (mul n0 n0) (mul n1 n1)) (mul n0 n0))
        (add (add (mul n0 n1) (mul n0 n0)) (mul n1 n0))) = Proved

theorem closed_cross_law_application
  : Equal OrdResult (orient p00 p10 p01)
      (compare
        (add (add (mul n0 n0) (mul n1 n1)) (mul n0 n0))
        (add (add (mul n0 n1) (mul n0 n0)) (mul n1 n0))) =
    orient_cross n0 n0 n1 n0 n0 n1
```

### `stdlib/geometry/orientation-closed-outcomes`

- spec: §18.1.
- promise class: durable invariant.
- given: the counter-clockwise, clockwise, and zero-free collinear triples.
- expect: `Gt`, `Lt`, and `Eq`, with cross sums `1` and `0` for the
  first triple and `11` and `11` for the last.
- why: these closed outcomes distinguish the specified orientation from
  a constant result and from comparing the sums in reverse.

### `stdlib/geometry/orientation-closed-laws`

- spec: §18.2.
- promise class: durable invariant.
- given: the closed triples in the accepted client.
- expect: `Proved` for each closed law statement and a successful client
  application of each public law: `orient_swap`, `orient_rotation`,
  `orient_repeated_point`, and `orient_cross`.
- why: the `Proved` rows pin the selected outcomes, while the separate
  applications exercise the exported law interfaces.

## Isolated refusals

For each refusal, start from a fresh copy of the complete accepted client
above and append only its theorem. The expected `KernelRejected
TypeMismatch` must be at that added theorem. Do not append both refusals
to one source.

### `stdlib/geometry/refuse-ccw-as-lt`

- spec: §18.1.
- promise class: durable invariant.
- given: the accepted counter-clockwise triple.
- expect: reject its claim that the result is `Lt`.
- why: the accepted client establishes the same triple's result is `Gt`.

```ken
theorem refuse_ccw_as_lt
  : Equal OrdResult (orient p00 p10 p01) Lt = Proved
```

### `stdlib/geometry/refuse-noncyclic-rotation`

- spec: §18.2.
- promise class: durable invariant.
- given: the accepted counter-clockwise triple and its lawful cyclic
  rotation.
- expect: reject the claim that swapping only the first two points
  preserves the orientation.
- why: the accepted client establishes that the swapped triple is
  clockwise, while the original is counter-clockwise.

```ken
theorem refuse_reverse_as_rotation
  : Equal OrdResult (orient p00 p10 p01) (orient p10 p00 p01) = Proved
```

## Evidence boundary

This seed is based on approved Spec child
`60b818219b4568f198f88d6205b701351c6cc2dd` and product candidate
`15ba3c19437805d3b6bd16393a1cbda12f5acb86` (tree
`9bbf96e88425cf01132ffe942ac4407f6e6d3d53`). On seed-child base
`2385f49f600a838e0af889011ecb8fd78699d1b1`, local `ken check` accepted
the complete accepted client against the product candidate's catalog
roots. Each refusal, checked from a fresh copy of that accepted client,
returned `KernelRejected TypeMismatch` with its span in the appended
theorem. These are local seed checks, not official conformance execution.
The official black-box conformance runner is not implemented on this
base. No package test, CI, publication, or landing result is claimed.
