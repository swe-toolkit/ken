# Box containment by corners

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §3.

At candidate base `4dcdd33fe1783985a033efae8968b0a790ec5b37`, the
`Algorithm.Geometry.NatBox` package does not yet include the §3 `inside`
operation or its seven law exports. This is an authored contract oracle,
not evidence of a delivered implementation or execution. Architect D0
(`evt_117jpaxkqd01k`) reported that a scratch package and client checked,
and that the strictly-smaller-box claim was refused with `TypeMismatch`.
That is scratch-only evidence, not execution of this seed, and it
establishes no trust measurement for this candidate. No exact-seed
execution, shipped harness, CI, publication, or landing is claimed.

The corners and expected values below follow §3's two comparisons on
both axes. `inner=(1,2)–(3,4)` is inside `outer=(0,0)–(6,6)`; the
witness is `(2,3)`. The overlapping boxes are `left_box=(0,1)–(3,4)`
and `right_box=(2,0)–(5,3)`: their intersection is `(2,1)–(3,3)`
and their hull is `(0,0)–(5,4)`. `strict_small_box` is the point box
`(2,3)–(2,3)`, strictly narrower than `inner` on both axes.

Every case promises a **durable invariant**. Closed instances complement
the generic theorem obligations; they do not prove those obligations.

## Shared fixtures and positive client

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, contains, intersect, hull, inside,
   inside_sound, inside_refl, intersect_inside_left,
   intersect_inside_right, inside_hull_left, inside_hull_right,
   inside_complete)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const n6 : Nat = Suc n5

const outer : Box = MkBox (MkPoint n0 n0) (MkPoint n6 n6)
const inner : Box = MkBox (MkPoint n1 n2) (MkPoint n3 n4)
const witness : Point = MkPoint n2 n3
const strict_small_box : Box = MkBox witness witness

const left_box : Box = MkBox (MkPoint n0 n1) (MkPoint n3 n4)
const right_box : Box = MkBox (MkPoint n2 n0) (MkPoint n5 n3)

const inverted_small : Box = MkBox (MkPoint n3 n3) (MkPoint n2 n2)
const inverted_wide : Box = MkBox (MkPoint n6 n6) (MkPoint n5 n5)
const separated_left : Box = MkBox (MkPoint n0 n0) (MkPoint n1 n1)
const separated_right : Box = MkBox (MkPoint n2 n2) (MkPoint n3 n3)

theorem nested_inside_value :
  Equal Bool (inside inner outer) True = Proved

theorem witness_in_inner :
  Equal Bool (contains inner witness) True = Proved

theorem witness_in_outer :
  Equal Bool (contains outer witness) True = Proved

theorem nested_sound_instance :
  IsTrue (contains outer witness) =
  inside_sound inner outer witness Proved Proved

theorem nested_complete_instance :
  IsTrue (inside inner outer) =
  inside_complete inner outer witness Proved
    (λp. λh. inside_sound inner outer p Proved h)

theorem strict_small_inside_inner_value :
  Equal Bool (inside strict_small_box inner) True = Proved

theorem inner_not_inside_strict_small_value :
  Equal Bool (inside inner strict_small_box) False = Proved

theorem inner_inside_itself_value :
  Equal Bool (inside inner inner) True = Proved

theorem inner_inside_itself_law :
  IsTrue (inside inner inner) = inside_refl inner

theorem inverted_small_inside_outer_value :
  Equal Bool (inside inverted_small outer) True = Proved

theorem inverted_wide_not_inside_inner_value :
  Equal Bool (inside inverted_wide inner) False = Proved

theorem inverted_self_law :
  IsTrue (inside inverted_small inverted_small) =
  inside_refl inverted_small

theorem overlap_intersection_inside_left_value :
  Equal Bool (inside (intersect left_box right_box) left_box) True = Proved

theorem overlap_intersection_inside_right_value :
  Equal Bool (inside (intersect left_box right_box) right_box) True = Proved

theorem overlap_intersection_inside_left_law :
  IsTrue (inside (intersect left_box right_box) left_box) =
  intersect_inside_left left_box right_box

theorem overlap_intersection_inside_right_law :
  IsTrue (inside (intersect left_box right_box) right_box) =
  intersect_inside_right left_box right_box

theorem overlap_hull_contains_left_value :
  Equal Bool (inside left_box (hull left_box right_box)) True = Proved

theorem overlap_hull_contains_right_value :
  Equal Bool (inside right_box (hull left_box right_box)) True = Proved

theorem overlap_hull_contains_left_law :
  IsTrue (inside left_box (hull left_box right_box)) =
  inside_hull_left left_box right_box

theorem overlap_hull_contains_right_law :
  IsTrue (inside right_box (hull left_box right_box)) =
  inside_hull_right left_box right_box

theorem inverted_intersection_inside_left_value :
  Equal Bool
    (inside (intersect separated_left separated_right) separated_left)
    True = Proved

theorem inverted_intersection_inside_right_value :
  Equal Bool
    (inside (intersect separated_left separated_right) separated_right)
    True = Proved

theorem inverted_intersection_inside_left_law :
  IsTrue (inside (intersect separated_left separated_right) separated_left) =
  intersect_inside_left separated_left separated_right

theorem inverted_intersection_inside_right_law :
  IsTrue (inside (intersect separated_left separated_right) separated_right) =
  intersect_inside_right separated_left separated_right
```

## Nested closed boxes: soundness and witnessed completeness

### stdlib/geometry/box-containment-nested

- spec: `spec/50-stdlib/67-geometry.md` §§3.1–3.2.
- promise class: durable invariant.
- given: `inner=(1,2)–(3,4)` inside `outer=(0,0)–(6,6)` and
  `witness=(2,3)` in both.
- expect: `inside inner outer` is `True`; `inside_sound` carries the
  witness to `outer`; `inside_complete` is instantiated with that
  witness and pointwise use of `inside_sound`.
- measured: one closed containment result and applications of the
  generic soundness and witnessed-completeness laws.
- claimed: the public law types support these valid nested-box cases.
- gap: finite instances do not establish the generic theorems.
- why: the witness is a concrete member of the inner box, as required
  by the premise of `inside_complete`.

## Overlapping boxes: intersection lies inside both operands

### stdlib/geometry/box-containment-overlap-intersection

- spec: `spec/50-stdlib/67-geometry.md` §3.2.
- promise class: durable invariant.
- given: `left_box=(0,1)–(3,4)` and
  `right_box=(2,0)–(5,3)`, with nonempty intersection
  `(2,1)–(3,3)`.
- expect: the intersection is inside each operand, both by direct
  closed values and by `intersect_inside_left` and
  `intersect_inside_right`.
- measured: both directions on the same overlapping pair.
- claimed: intersection containment is not limited to one operand.
- gap: the closed pair does not establish the generic laws.
- why: neither input contains the other, so the two directions remain
  observable independently.

## Overlapping boxes: both operands lie inside the hull

### stdlib/geometry/box-containment-overlap-hull

- spec: `spec/50-stdlib/67-geometry.md` §3.2.
- promise class: durable invariant.
- given: the same overlapping `left_box` and `right_box`; their hull is
  `(0,0)–(5,4)`.
- expect: each operand is inside the hull, by direct closed values and
  the corresponding `inside_hull_left` and `inside_hull_right` laws.
- measured: both operand-to-hull directions on the same pair.
- claimed: the hull laws preserve either operand.
- gap: these two applications do not establish the generic laws.
- why: the overlap fixture has distinct bounds on both boxes, so each
  direction exercises its own operand.

## Self-containment and corner-based inverted boxes

### stdlib/geometry/box-containment-self-and-inverted

- spec: `spec/50-stdlib/67-geometry.md` §§3.1–3.2.
- promise class: durable invariant.
- given: `inner` with itself; inverted boxes `inverted_small` and
  `inverted_wide`; and the empty intersection of `separated_left` and
  `separated_right`.
- expect: self-containment is true by value and `inside_refl`;
  `inverted_small` is inside `outer` by its corners, while
  `inverted_wide` is not inside `inner`; the inverted intersection is
  inside both separated operands by value and by the intersection laws.
- measured: closed corner comparisons and reflexivity/intersection law
  instances, without a validity premise or empty-box special case.
- claimed: `inside` compares corners, including inverted boxes and an
  empty intersection.
- gap: the finite boxes do not replace the generic law obligations.
- why: the selected inverted examples separate corner-wise containment
  from an empty-box-as-set interpretation.

## Refuse an inner box inside a strictly smaller box

### stdlib/geometry/reject-inner-inside-strict-small-box

- spec: `spec/50-stdlib/67-geometry.md` §§3.1–3.2.
- promise class: durable invariant.
- given: `inner=(1,2)–(3,4)` and
  `strict_small_box=(2,3)–(2,3)`, with the point box properly
  contained in `inner` on both axes.
- expect: the positive controls below pass; the claim that `inner` is
  inside `strict_small_box` is refused at
  `reject_inner_inside_strict_small_box`.
- measured: same-box self-containment, `inner` inside `outer`, and
  `strict_small_box` inside both `inner` and `outer` before the target.
- claimed: containment is directional; a known inner box cannot fit
  inside this strictly smaller point box.
- gap: one fixed pair does not prove the generic corner definition.
- why: the target uses the same two boxes as the controls, in the
  opposite containment direction. The required lower comparison fails
  (`2 ≤ 1`); an unrelated false claim cannot satisfy this refusal.

Each refusal block is appended alone to the positive client above.
The preceding controls must elaborate before the final theorem is
checked.

```ken
-- Refuse: the same two boxes fit only in the smaller-to-inner direction.
theorem reject_control_inner_self :
  IsTrue (inside inner inner) = Proved

theorem reject_control_inner_inside_outer :
  IsTrue (inside inner outer) = Proved

theorem reject_control_small_inside_inner :
  IsTrue (inside strict_small_box inner) = Proved

theorem reject_control_small_inside_outer :
  IsTrue (inside strict_small_box outer) = Proved

theorem reject_inner_inside_strict_small_box :
  IsTrue (inside inner strict_small_box) = Proved
```
