# Box intersection and hull distribute on natural boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §9.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INTERSECT-HULL-DISTRIBUTIVITY.md`.

The fixed boxes are `a=(1,1)–(4,4)`, `b=(3,0)–(6,2)`, and
`c=(0,3)–(2,3)`. The two sides of the first law both compute to
`(1,1)–(4,3)`, which is not any input box. The two sides of the second
law both compute to `a`: `intersect b c` is the inverted box
`(3,3)–(2,2)`. The accepted client directly applies both Nat Order
selectors at their generic `Equal Nat` types, then applies both Box laws
and checks each side independently with `Refl`. These closed values
delimit the generic laws; they do not prove them.

## Accepted client

```ken
import Data.Numeric.Nat.Order (min, max)

import Algorithm.Geometry.NatBox
  (Box, MkPoint, MkBox, intersect, hull, intersect_hull_distrib,
   hull_intersect_distrib)

const zero : Nat = Zero

const one : Nat = Suc zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const six : Nat = Suc (Suc (Suc three))

theorem min_distrib_client (m : Nat) (n : Nat) (k : Nat)
  : Equal Nat (min m (max n k)) (max (min m n) (min m k)) =
  (proof distrib for min) m n k

theorem max_distrib_client (m : Nat) (n : Nat) (k : Nat)
  : Equal Nat (max m (min n k)) (min (max m n) (max m k)) =
  (proof distrib for max) m n k

const box_a : Box = MkBox (MkPoint one one) (MkPoint four four)

const box_b : Box = MkBox (MkPoint three zero) (MkPoint six two)

const box_c : Box = MkBox (MkPoint zero three) (MkPoint two three)

const first_result : Box = MkBox (MkPoint one one) (MkPoint four three)

const inverted_bc : Box = MkBox (MkPoint three three) (MkPoint two two)

const mixed_right_result : Box = MkBox (MkPoint zero one) (MkPoint four three)

theorem first_left_value
    : Equal Box (intersect box_a (hull box_b box_c)) first_result =
  Refl

theorem first_right_value
    : Equal Box
        (hull (intersect box_a box_b) (intersect box_a box_c))
        first_result =
  Refl

theorem first_distributes_by_law
    : Equal Box
        (intersect box_a (hull box_b box_c))
        (hull (intersect box_a box_b) (intersect box_a box_c)) =
  intersect_hull_distrib box_a box_b box_c

theorem inverted_intersection_value
    : Equal Box (intersect box_b box_c) inverted_bc =
  Refl

theorem second_left_value
    : Equal Box (hull box_a (intersect box_b box_c)) box_a =
  Refl

theorem second_right_value
    : Equal Box
        (intersect (hull box_a box_b) (hull box_a box_c))
        box_a =
  Refl

theorem second_distributes_by_law
    : Equal Box
        (hull box_a (intersect box_b box_c))
        (intersect (hull box_a box_b) (hull box_a box_c)) =
  hull_intersect_distrib box_a box_b box_c

theorem same_form_true_control
    : Equal Box
        (intersect box_a (hull box_b box_c))
        (hull (intersect box_a box_b) (intersect box_a box_c)) =
  Refl

theorem mixed_right_value
    : Equal Box (hull (intersect box_a box_b) box_c) mixed_right_result =
  Refl
```

### `stdlib/geometry/nat-order-distribution-selectors`

- spec: §9.1.
- promise class: durable invariant.
- given: the generic `min_distrib_client` and `max_distrib_client`
  declarations in the accepted client, for arbitrary `m`, `n`, and `k`.
- expect: each public selector checks at its exact stated `Equal Nat`
  type: `min_distrib_client` applies `distrib for min`, and
  `max_distrib_client` applies `distrib for max`.
- measured: when run, both arbitrary-variable applications typecheck
  against their stated Nat equality types.
- why: the public Nat Order API must be exercised directly; the Box
  theorem applications do not stand in for these selector clients.

### `stdlib/geometry/box-intersection-hull-distribution`

- spec: §9.2.
- promise class: durable invariant.
- given: the fixed three boxes above, including their inverted
  `intersect box_b box_c` result.
- expect: `intersect_hull_distrib` and `hull_intersect_distrib` prove
  the two stated structural equalities.
- measured: when run, each side is independently checked by `Refl`,
  and each public law is applied to the same closed triple.
- claimed: both laws hold for every three boxes, including inverted
  bounds; the finite cases do not establish either generic theorem.
- gap: closed instances do not establish the generic proof bodies or
  catalog exports.
- why: `Equal Box` compares stored corners. The first law's result is
  distinct from all three inputs, while the second exercises an
  inverted intermediate without treating boxes as point sets.

## One isolated refused claim

Start from a fresh, complete copy of the accepted client above,
including `same_form_true_control` and `mixed_right_value`. Append only
the false mixed regrouping below. Its left side computes to
`(1,1)–(4,3)` and its right side to `(0,1)–(4,3)`. They differ only
at the lower x coordinate. The true control accepts the same
`Equal Box`/`Refl` form for the actual first-law equality; both sides
of the false claim are independently shown to compute before it is
attempted.

### `stdlib/geometry/reject-mixed-intersection-hull-regrouping`

- spec: §9.2.
- promise class: durable invariant.
- given: the same accepted triple and independently checked mixed
  right side.
- expect: the appended `seed_neg` is refused at its false equality.
- measured: when run, the accepted prefix and same-form true control
  pass; the false proposition differs at the lower x coordinate.
- claimed: the stated distribution law does not license this mixed
  regrouping.
- gap: this refusal does not challenge either generic distribution
  theorem.
- why: the valid distribution law does not license dropping the inner
  `intersect` from its right side.

```ken
theorem seed_neg
    : Equal Box
        (intersect box_a (hull box_b box_c))
        (hull (intersect box_a box_b) box_c) =
  Refl
```

## Evidence boundary

This seed was first authored on the spec-only child, before the §9
catalog exports were present; no Ken check or package test was run on
that authoring child.

On exact parent `2aed54397a23e0aa8f21f090af35ff1a6bd9399f`, Foundation QA
ran the named `cat_geometry_box_intersect_hull_distributivity` target:
2/2 passed. QA then changed only `seed_neg`'s false right side to the
true right side of `intersect_hull_distrib`; the mutated run had 1
passing test, and the other failed at the intended refusal assertion. After restoration from
the candidate blob,
the target passed 2/2 again. The `include_str!` target rebuilt; the old-
RHS anchor count went 1→0 and the true-neighbor count went 2→3. This
population-side mutation distinguished the false regrouping from its
nearest true neighbor. These are package-test results, not an official
conformance-runner result. Architect's
AC-0 results remain scratch evidence. The closed examples do not replace
the checked generic proofs. No CI, publication, landing or landed trust
is claimed.
