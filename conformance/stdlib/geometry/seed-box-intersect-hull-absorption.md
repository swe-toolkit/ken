# Intersection and hull absorb each other on natural boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §8.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INTERSECT-HULL-ABSORPTION.md`.

The two main fixtures overlap, are distinct, and neither contains the
other. Their hull is `(1,0)–(6,4)` and their intersection is
`(3,1)–(4,2)`. The accepted client independently checks those corners,
both absorption results, and applications of the public Nat and box
laws. A second fixture has an inverted first box; both laws still return
that stored box. Finite examples do not establish the generic theorems.

## Accepted client

`box_a` spans `(1,1)–(4,4)` and `box_b` spans `(3,0)–(6,2)`. Neither
contains the other, and their intersection is nonempty. `inverted_a`
spans `(4,4)–(1,1)`; it is a valid `Box` despite its inverted bounds.

```ken
import Data.Numeric.Nat.Order (min, max)

import Algorithm.Geometry.NatBox
  (Box, MkPoint, MkBox, intersect, hull, intersect_hull_absorb, hull_intersect_absorb)

const zero : Nat = Zero

const one : Nat = Suc zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const six : Nat = Suc (Suc (Suc three))

const box_a : Box = MkBox (MkPoint one one) (MkPoint four four)

const box_b : Box = MkBox (MkPoint three zero) (MkPoint six two)

const inverted_a : Box = MkBox (MkPoint four four) (MkPoint one one)

const hull_ab : Box = MkBox (MkPoint one zero) (MkPoint six four)

const intersection_ab : Box = MkBox (MkPoint three one) (MkPoint four two)

theorem nat_min_absorption_value : Equal Nat (min one (max one three)) one = Proved

theorem nat_min_absorption_law : Equal Nat (min one (max one three)) one =
  (proof absorb for min) one three

theorem nat_max_absorption_value : Equal Nat (max four (min four two)) four = Proved

theorem nat_max_absorption_law : Equal Nat (max four (min four two)) four =
  (proof absorb for max) four two

theorem hull_ab_value : Equal Box (hull box_a box_b) hull_ab = Refl

theorem intersection_ab_value : Equal Box (intersect box_a box_b) intersection_ab = Refl

theorem intersect_hull_ab_value : Equal Box (intersect box_a (hull box_a box_b)) box_a = Refl

theorem hull_intersect_ab_value : Equal Box (hull box_a (intersect box_a box_b)) box_a = Refl

theorem intersect_hull_ab_law : Equal Box (intersect box_a (hull box_a box_b)) box_a =
  intersect_hull_absorb box_a box_b

theorem hull_intersect_ab_law : Equal Box (hull box_a (intersect box_a box_b)) box_a =
  hull_intersect_absorb box_a box_b

theorem inverted_intersection_value : Equal Box (intersect inverted_a box_b) inverted_a = Refl

theorem inverted_hull_value : Equal Box (hull inverted_a box_b) box_b = Refl

theorem inverted_intersect_hull_value
    : Equal Box (intersect inverted_a (hull inverted_a box_b)) inverted_a =
  Refl

theorem inverted_hull_intersect_value
    : Equal Box (hull inverted_a (intersect inverted_a box_b)) inverted_a =
  Refl

theorem inverted_intersect_hull_law
    : Equal Box (intersect inverted_a (hull inverted_a box_b)) inverted_a =
  intersect_hull_absorb inverted_a box_b

theorem inverted_hull_intersect_law
    : Equal Box (hull inverted_a (intersect inverted_a box_b)) inverted_a =
  hull_intersect_absorb inverted_a box_b

theorem same_form_true_control : Equal Box (intersect (hull box_a box_b) box_b) box_b = Refl
```

### `stdlib/geometry/nat-absorption`

- spec: §8.1.
- promise class: durable invariant.
- given: natural values `one`, `two`, `three`, and `four`.
- expect: `min one (max one three) = one` and
  `max four (min four two) = four`; both public Nat laws apply.
- measured: when run, each closed value uses `Proved` and each public
  law application proves the same equation.
- claimed: the public selectors return the left operand for the stated
  nested opposite extremum.
- gap: these closed values do not prove either generic law.
- why: the selected values differ on each side of the nested extrema.

### `stdlib/geometry/box-absorption`

- spec: §8.2.
- promise class: durable invariant.
- given: distinct overlapping `box_a` and `box_b`, and the inverted
  `inverted_a` paired with `box_b`.
- expect: the direct hull and intersection corners match their listed
  results; both laws return the left box for both fixtures.
- measured: when run, closed `Equal Box` controls use `Refl`, and both
  public laws are separately applied to regular and inverted inputs.
- claimed: each operation pair restores the left box structurally,
  including this inverted-box instance.
- gap: closed inputs do not prove the unconditional generic theorems.
- why: the inverted fixture exercises the no-validity-premise boundary.

## Isolated refusal

Start from a fresh full copy of the accepted client above and append
only this theorem. The accepted `same_form_true_control` proves that
`intersect (hull box_a box_b) box_b` equals `box_b` with the same
`Equal Box` and `Refl` form. Since `box_a` and `box_b` have different
stored corners, the false equality to `box_a` must be refused at its
own `Refl`, not at an import or earlier declaration.

### `stdlib/geometry/reject-intersect-hull-right-operand-is-not-left`

- spec: §8.2.
- promise class: durable invariant.
- given: the accepted pair `box_a` and `box_b`.
- expect: reject the equality of `intersect (hull box_a box_b) box_b`
  with `box_a` at the appended claim.
- measured: when run, the same-form equality with `box_b` accepts;
  changing only the right-hand box to `box_a` makes the claim false.
- claimed: intersection of a hull with its right operand does not
  generally recover the distinct left operand.
- gap: this counterexample does not refute either §8 absorption law.
- why: the positive control fixes the expression and proof form; only
  the expected box differs.

```ken
theorem seed_neg : Equal Box (intersect (hull box_a box_b) box_b) box_a = Refl
```

## Evidence boundary

The §8 catalog exports are absent on the seed's base
`439fa7bb7cca6209a66e0a9861e6316ce0e5ae9b`. Using a targeted local
CLI build, I checked the authored client in a disposable overlay of
Architect's D0 patch (sha256
`440df27e96f94cb4796e0723cf280781dec2bce2ff2a6ac47b75e6408daca9d7`)
on exact spec tip `4d9e21b24f0a6c55e4873fad800c9d31f889ae78`. The full
accepted client passed `ken check`; the same client with only `seed_neg`
appended was refused with `E0038 TypeMismatch` at its `Refl`. This is
scratch D0 evidence, not execution against a Foundation catalog
candidate, official conformance-runner execution, CI, publication, landing
or landed trust. The finite cases do not prove the generic laws.
