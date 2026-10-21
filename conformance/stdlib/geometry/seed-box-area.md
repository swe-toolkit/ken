# Natural area of a closed box

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §4, approved in Decision
`dec_12asnhjab294b` on candidate
`5efe44c95fa8dde9d3109b814233f0095e0213ae`.

This source/content seed follows the approved §4 contract. Its candidate
base is `4577ba7e3a843ad6a4326e744928e31509bd7204`; §4 and the area
exports were not present at that base. The package-owned roots-loader
test now executes the positive and refused fences against the §4 exports
and compares the exact trusted-base identities. This is not an official
conformance-runner run, CI, publication or landing claim.

The closed expected values follow the contract's inclusive natural spans:
a point box has area 1, the box from `(0,0)` to `(2,2)` has area 9, and
an inversion on either axis gives area 0. The nested pair uses the same
outer 3-by-3 box and an inner 2-by-2 box, so `area_inside` has a positive
control while the strictly-larger reverse claim is false. Closed instances
complement the generic theorem obligations; they do not prove them.

## Shared fixtures and positive client

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)
import Algorithm.Geometry.NatBox
  (MkPoint, Box, MkBox, inside, area, area_inside,
   area_inverted_x, area_inverted_y)

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

const point_box : Box = MkBox (MkPoint n1 n1) (MkPoint n1 n1)
const outer_box : Box = MkBox (MkPoint n0 n0) (MkPoint n2 n2)
const inner_box : Box = MkBox (MkPoint n1 n1) (MkPoint n2 n2)
const x_inverted_box : Box = MkBox (MkPoint n4 n0) (MkPoint n2 n2)
const y_inverted_box : Box = MkBox (MkPoint n0 n4) (MkPoint n2 n2)

theorem point_box_area_value :
  Equal Nat (area point_box) n1 = Proved

theorem proper_box_area_value :
  Equal Nat (area outer_box) n9 = Proved

theorem inner_box_area_value :
  Equal Nat (area inner_box) n4 = Proved

theorem x_inverted_area_value :
  Equal Nat (area x_inverted_box) Zero = Proved

theorem x_inverted_area_law :
  Equal Nat (area x_inverted_box) Zero =
  area_inverted_x n4 n0 n2 n2 Proved

theorem y_inverted_area_value :
  Equal Nat (area y_inverted_box) Zero = Proved

theorem y_inverted_area_law :
  Equal Nat (area y_inverted_box) Zero =
  area_inverted_y n0 n4 n2 n2 Proved

theorem nested_inside_value :
  Equal Bool (inside inner_box outer_box) True = Proved

theorem nested_area_order_value :
  Equal Bool (leq_nat (area inner_box) (area outer_box)) True = Proved

theorem nested_area_inside_law :
  IsTrue (leq_nat (area inner_box) (area outer_box)) =
  area_inside inner_box outer_box Proved

theorem inner_is_not_strictly_larger_by_value :
  Equal Bool
    (leq_nat (Suc (area outer_box)) (area inner_box)) False = Proved
```

## Single-point and proper-box area

### stdlib/geometry/box-area-values

- spec: `spec/50-stdlib/67-geometry.md` §4.1.
- promise class: durable invariant.
- given: `point_box=(1,1)–(1,1)` and
  `outer_box=(0,0)–(2,2)`.
- expect: the inclusive point box has area 1 and the proper 3-by-3
  box has area 9.
- measured: when run, two direct `Equal Nat` results with fixed
  expected values.
- claimed: area counts inclusive natural-coordinate positions on both
  axes.
- gap: these closed values do not establish the generic area definition.
- why: the point and proper-box values distinguish inclusive spans from
  endpoint-distance or continuous-area interpretations.

## Inverted x and y axes have zero area

### stdlib/geometry/box-area-inverted-axes

- spec: `spec/50-stdlib/67-geometry.md` §§4.1–4.2.
- promise class: durable invariant.
- given: `x_inverted_box=(4,0)–(2,2)` and
  `y_inverted_box=(0,4)–(2,2)`.
- expect: each area is zero, and the corresponding public
  `area_inverted_x` and `area_inverted_y` law instance checks.
- measured: when run, direct zero-area results plus separate
  applications of both axis-specific laws; the unaffected axis is valid
  in each fixture.
- claimed: either inverted axis forces zero area, with no conflation
  between the x and y laws.
- gap: the two fixed instances do not establish either generic law.
- why: each fixture inverts only its named axis, so a defect on one axis
  cannot be hidden by the other.

## Area is monotone for a closed nested pair

### stdlib/geometry/box-area-nested-monotonicity

- spec: `spec/50-stdlib/67-geometry.md` §4.2.
- promise class: durable invariant.
- given: `inner_box=(1,1)–(2,2)` inside
  `outer_box=(0,0)–(2,2)`.
- expect: the area values are 4 and 9, the `inside` result is true,
  and `area_inside inner_box outer_box` establishes the corresponding
  `leq_nat` proposition.
- measured: when run, the fixed values, the corner-wise containment
  control, and the public monotonicity-law instance on the same pair.
- claimed: the contract's `inside` premise entails non-strict area
  monotonicity without an outer-validity premise.
- gap: this closed instance does not prove the generic law.
- why: the boxes have distinct positive areas and the inner box is
  nested, so the positive monotonicity control is non-degenerate.

## Refuse a strictly larger inner-area claim

For the refusal, start from a fresh copy of the accepted positive client
above and append only this fence. Expect the base alone to accept and
the appended input to reject at `reject_inner_area_strictly_larger`.
No particular diagnostic code is specified by §4.

### stdlib/geometry/reject-inner-area-strictly-larger

- spec: `spec/50-stdlib/67-geometry.md` §4.2.
- promise class: durable invariant.
- given: the same `inner_box` and `outer_box`; the positive client
  establishes areas 4 and 9, nesting, and the false value of
  `leq_nat (Suc (area outer_box)) (area inner_box)`.
- expect: the claim that the inner area is strictly larger is refused
  at `reject_inner_area_strictly_larger`.
- measured: when run, refusal of the false strict comparison, with the
  direct false-value control and accepted monotonicity instance on the
  same pair.
- claimed: an inner box cannot have strictly greater area than its
  containing outer box under the specified `inside` relation.
- gap: this fixed pair does not prove a general strict-area result.
- why: `Suc outer_area ≤ inner_area` expresses strict greater-than for
  `Nat`; the closed values make the refusal attributable to that claim.

```ken
-- Reject: the inner box does not have strictly greater area.
theorem reject_inner_area_strictly_larger :
  IsTrue (leq_nat (Suc (area outer_box)) (area inner_box)) = Proved
```

This seed specifies source-shaped observations. Its two source fences
are executed by the package-owned roots-loader test. Official
conformance-runner execution and CI remain unmeasured; publication
and landing are not asserted.
