# Intersection and hull preserve box inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §17.
Frame: `docs/program/issues/SPEC-GEOMETRY-BOX-INTERSECT-HULL-MONOTONE.md`.

These cases apply the two public monotonicity laws to closed boxes. The
normal-box controls exercise both laws with two proved premises. The additional
inverted-box controls put inverted boxes in both premise positions and their
targets, matching §17's unconditional corner-wise contract. The closed
applications test the interface; they do not replace the generic checked
bodies.

The four refusals independently fail one premise position in one law while
the other premise remains proved. Each has a same-form accepted control that
changes only the failed target box from `short` to `big`. Each refusal starts
from a fresh copy of the complete accepted client and appends only its own
claim; do not accumulate the refusals in one source.

## Accepted client

```ken
import Algorithm.Geometry.NatBox
  (Box, MkPoint, MkBox, inside, intersect, hull,
   intersect_monotone, hull_monotone)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const tall : Box = MkBox (MkPoint Zero Zero) (MkPoint three two)
const short : Box = MkBox (MkPoint Zero Zero) (MkPoint three one)
const thin : Box = MkBox (MkPoint one one) (MkPoint two one)

const inverted_left : Box = MkBox (MkPoint four one) (MkPoint one two)
const inverted_left_outer : Box = MkBox (MkPoint three Zero) (MkPoint two three)
const inverted_right : Box = MkBox (MkPoint five one) (MkPoint Zero two)
const inverted_right_outer : Box = MkBox (MkPoint four Zero) (MkPoint one three)

theorem small_inside_big : IsTrue (inside small big) = Proved
theorem small_inside_tall : IsTrue (inside small tall) = Proved
theorem small_inside_small : IsTrue (inside small small) = Proved
theorem short_inside_short : IsTrue (inside short short) = Proved

theorem small_not_inside_short
    : Equal Bool (inside small short) False =
  Proved

theorem inside_tall_short_is_false
    : Equal Bool (inside tall short) False =
  Proved

theorem small_not_inside_thin
    : Equal Bool (inside small thin) False =
  Proved

theorem intersect_short_small_value
    : Equal Box (intersect short small) thin =
  Refl

theorem intersect_small_short_value
    : Equal Box (intersect small short) thin =
  Refl

theorem hull_small_short_value
    : Equal Box (hull small short) tall =
  Refl

theorem hull_short_small_value
    : Equal Box (hull short small) tall =
  Refl

theorem intersect_monotone_normal
    : IsTrue (inside (intersect small small) (intersect big tall)) =
  intersect_monotone small small big tall small_inside_big small_inside_tall

theorem hull_monotone_normal
    : IsTrue (inside (hull small small) (hull big tall)) =
  hull_monotone small small big tall small_inside_big small_inside_tall

theorem intersect_first_premise_control
    : IsTrue (inside (intersect small small) (intersect big small)) =
  intersect_monotone small small big small small_inside_big small_inside_small

theorem intersect_second_premise_control
    : IsTrue (inside (intersect small small) (intersect small big)) =
  intersect_monotone small small small big small_inside_small small_inside_big

theorem hull_first_premise_control
    : IsTrue (inside (hull small short) (hull big short)) =
  hull_monotone small short big short small_inside_big short_inside_short

theorem hull_second_premise_control
    : IsTrue (inside (hull short small) (hull short big)) =
  hull_monotone short small short big short_inside_short small_inside_big

theorem inverted_left_inside_outer
    : IsTrue (inside inverted_left inverted_left_outer) =
  Proved

theorem inverted_right_inside_outer
    : IsTrue (inside inverted_right inverted_right_outer) =
  Proved

theorem intersect_monotone_inverted
    : IsTrue
        (inside
          (intersect inverted_left inverted_right)
          (intersect inverted_left_outer inverted_right_outer)) =
  intersect_monotone
    inverted_left
    inverted_right
    inverted_left_outer
    inverted_right_outer
    inverted_left_inside_outer
    inverted_right_inside_outer

theorem hull_monotone_inverted
    : IsTrue
        (inside
          (hull inverted_left inverted_right)
          (hull inverted_left_outer inverted_right_outer)) =
  hull_monotone
    inverted_left
    inverted_right
    inverted_left_outer
    inverted_right_outer
    inverted_left_inside_outer
    inverted_right_inside_outer
```

### `stdlib/geometry/box-intersect-monotone`

- spec: §17.
- promise class: durable invariant.
- given: `small` is inside `big` and `tall`.
- expect: `intersect_monotone` proves the intersection of the smaller
  operands is inside the intersection of the larger operands.
- why: the application supplies both premises in the specified order;
  the closed example does not stand in for the generic proof.

### `stdlib/geometry/box-hull-monotone`

- spec: §17.
- promise class: durable invariant.
- given: the same two proved inclusions as the intersection case.
- expect: `hull_monotone` proves the hull of the smaller operands is
  inside the hull of the larger operands.
- why: both ordered premises are used in a closed application of the
  public law.

### `stdlib/geometry/box-monotone-inverted-operands`

- spec: §17.
- promise class: durable invariant.
- given: each inverted box is inside its corresponding inverted outer
  box by the stored-corner comparison.
- expect: both public laws prove their conclusions for four inverted
  boxes.
- why: this exercises the stated no-validity-premise boundary; inverted
  boxes are not replaced by a point-set equality.

## Isolated refusals

For each case, begin from a fresh copy of the entire accepted client above
and append only the indicated theorem. The corresponding accepted control
has the same law application shape and conclusion form, with only the
failed target box changed from `short` to `big`. The direct value controls
in the accepted client establish that the failed premise and the refused
conclusion are both false. Expect rejection at the appended `Proved`
argument for the one false premise, not at an import, fixture, or earlier
declaration.

### `stdlib/geometry/reject-intersect-monotone-first-premise`

- spec: §17.
- promise class: durable invariant.
- given: `inside small short` is false; the second premise
  `inside small small` is true.
- expect: reject the attempted first-premise proof in the application of
  `intersect_monotone`; the resulting inclusion is also false.
- why: only the first premise differs from its accepted control.

```ken
theorem reject_intersect_monotone_first_premise
    : IsTrue (inside (intersect small small) (intersect short small)) =
  intersect_monotone small small short small Proved small_inside_small
```

### `stdlib/geometry/reject-intersect-monotone-second-premise`

- spec: §17.
- promise class: durable invariant.
- given: `inside small small` is true; `inside small short` is false.
- expect: reject the attempted second-premise proof in the application
  of `intersect_monotone`; the resulting inclusion is also false.
- why: only the second premise differs from its accepted control.

```ken
theorem reject_intersect_monotone_second_premise
    : IsTrue (inside (intersect small small) (intersect small short)) =
  intersect_monotone small small small short small_inside_small Proved
```

### `stdlib/geometry/reject-hull-monotone-first-premise`

- spec: §17.
- promise class: durable invariant.
- given: `inside small short` is false; the second premise
  `inside short short` is true.
- expect: reject the attempted first-premise proof in the application of
  `hull_monotone`; the resulting inclusion is also false.
- why: only the first premise differs from its accepted control.

```ken
theorem reject_hull_monotone_first_premise
    : IsTrue (inside (hull small short) (hull short short)) =
  hull_monotone small short short short Proved short_inside_short
```

### `stdlib/geometry/reject-hull-monotone-second-premise`

- spec: §17.
- promise class: durable invariant.
- given: `inside short short` is true; `inside small short` is false.
- expect: reject the attempted second-premise proof in the application
  of `hull_monotone`; the resulting inclusion is also false.
- why: only the second premise differs from its accepted control.

```ken
theorem reject_hull_monotone_second_premise
    : IsTrue (inside (hull short small) (hull short short)) =
  hull_monotone short small short short short_inside_short Proved
```

## Evidence boundary

CV authored these cases from §17 and the public interface. The seed branch
starts at `2345d4178`, before the separately authorized catalog addition;
its imports of `intersect_monotone` and `hull_monotone` therefore require the
assembled candidate containing that catalog half. A temporary, uncommitted
`ElabEnv` probe accepted the corrected client and observed all four
fresh-prefix refusals as `KernelRejected TypeMismatch` at their own theorem.
This scratch result is not official conformance-runner execution. CV has not
run CI or performed a publication/landing check for this seed.
