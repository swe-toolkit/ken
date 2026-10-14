# Exact natural points and closed boxes

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §1.

These are **authored source-shaped closed behavioral oracles** for the
prospective `Algorithm.Geometry.NatBox` module. The D0 development
checks the six generic laws in scratch with a two-declaration
visibility adjustment to `Data.Numeric.Nat.Order`; that adjustment
and the explicit point/box constructor exports are **not** landed
catalog changes. The positive source below is a separate client of
the scratch module. Every case promises a **durable invariant**:
correctness is about `Nat` values, membership, and structural box
corners, not the particular spelling of a scratch proof. A finite
instance of a generic theorem does not by itself prove that theorem.

The exact Architect D0 source (SHA256
`3314c80dd92933e95d16ee36776a65043fbead2cb9248fbe472f539fd304a92e`)
checked rc=0 only with `greatest for min` and `least for max` made
public in a **scratch copy** of Nat Order (SHA256
`1739e867957b711aaf958efc1c1dc36e6a80609c96b851c5a3f327a37bd6c25e`).
With the landed private Order instead, this same D0 rejected at
`UnboundName max::least`. A separate public client first rejected
at `UnboundName MkPoint`; exporting only `MkPoint` then rejected at
`UnboundName MkBox`. Adding both exports to the scratch D0 module
(SHA256
`b5fface470b5878b3d54944d80c3a0d29c5f80232434d0c8481530385d640bbc`)
made that client and the unchanged generic proof bodies check rc=0.
The positive block below was extracted verbatim (SHA256
`8d7d8a7b3a19348c821fada9b270f97d2a3306d50d0ab969427fd298b9a039f5`):
all **22** closed assertions check, rc=0. Each of the five negative
blocks, appended alone to the same positive client, rejected rc=1:
one-box-only membership, a fabricated second membership premise and
an empty intersection's alleged member each give
`KernelRejected TypeMismatch`; wrong hull corners give `TypeMismatch`
at `Refl`; wrong distance gives `KernelRejected TypeMismatch`. An
independent scratch mutation deleting the vertical condition from
`contains` (SHA256
`62997a645180ca99fecc6077246e8b573ac65cc969da8ffe226348bddb398d57`)
rejected the generic `intersect_sound` body rc=1
`KernelRejected TypeMismatch`; that mutation does not stand in for
the fixed-value seed oracles.
These checks used `ken check` from base `813346f95` after a targeted
`scripts/ken-cargo build -p ken-cli --bin ken` (rc=0). None measures
`trusted_base()` delta, runs a shipped conformance suite or Rust
test, establishes catalog delivery, or establishes CI.

## Shared fixtures and positive assertions

`bij` spans the closed square from `(i,i)` to `(j,j)`.
`b02 ∩ b13` has corners `(1,1)` and `(2,2)`; `b01 ∩ b23`
has inverted corners `(2,2)` and `(1,1)` and is empty without an
`Option` or special empty constructor. `(0,0)` belongs to `b02`
but not `b13` and therefore cannot belong to their intersection.
The expected corners and the distance five are built from fixed
`Nat` literals, not computed by the operation whose behavior they
assert. The six `seed_law_*` declarations instantiate the generic
D0 proofs separately from the direct operation oracles. The
`And` in the soundness result is Ken's ambient Ω-valued
conjunction, **not** `Core.Logic.And`'s Type-valued carrier;
importing the latter under the same spelling changes the goal.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, axis_contains, contains, intersect,
   hull, manhattan, intersect_sound, intersect_complete,
   hull_contains_left, hull_contains_right, manhattan_symm,
   manhattan_self)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4

const b02 : Box = MkBox (MkPoint n0 n0) (MkPoint n2 n2)
const b13 : Box = MkBox (MkPoint n1 n1) (MkPoint n3 n3)
const b01 : Box = MkBox (MkPoint n0 n0) (MkPoint n1 n1)
const b23 : Box = MkBox (MkPoint n2 n2) (MkPoint n3 n3)
const only_first : Point = MkPoint n0 n0
const in_both : Point = MkPoint n1 n2

theorem axis_lower_in : Equal Bool (axis_contains n1 n2 n1) True = Proved
theorem axis_upper_in : Equal Bool (axis_contains n1 n2 n2) True = Proved
theorem axis_below_out : Equal Bool (axis_contains n1 n2 n0) False = Proved
theorem axis_above_out : Equal Bool (axis_contains n1 n2 n3) False = Proved
theorem axis_inverted_out : Equal Bool (axis_contains n2 n1 n2) False = Proved

theorem contains_in : Equal Bool (contains b02 in_both) True = Proved
theorem contains_out : Equal Bool (contains b02 (MkPoint n3 n1)) False = Proved

theorem intersect_corners :
  Eq Box (intersect b02 b13)
    (MkBox (MkPoint n1 n1) (MkPoint n2 n2)) = Refl

theorem intersect_disjoint_corners :
  Eq Box (intersect b01 b23)
    (MkBox (MkPoint n2 n2) (MkPoint n1 n1)) = Refl

theorem empty_meet_excludes_lower_input_corner :
  Equal Bool (contains (intersect b01 b23) (MkPoint n1 n1)) False = Proved

theorem empty_meet_excludes_upper_input_corner :
  Equal Bool (contains (intersect b01 b23) (MkPoint n2 n2)) False = Proved

theorem only_first_inside : Equal Bool (contains b02 only_first) True = Proved
theorem only_first_outside_second :
  Equal Bool (contains b13 only_first) False = Proved
theorem only_first_outside_intersection :
  Equal Bool (contains (intersect b02 b13) only_first) False = Proved

theorem hull_corners :
  Eq Box (hull b01 b23)
    (MkBox (MkPoint n0 n0) (MkPoint n3 n3)) = Refl

theorem distance_value :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1)) n5 = Proved

theorem seed_law_intersect_complete :
  IsTrue (contains (intersect b02 b13) (MkPoint n2 n1)) =
  intersect_complete b02 b13 (MkPoint n2 n1) Proved Proved

theorem seed_law_intersect_sound :
  And (IsTrue (contains b02 in_both))
      (IsTrue (contains b13 in_both)) =
  intersect_sound b02 b13 in_both Proved

theorem seed_law_hull_left :
  IsTrue (contains (hull b01 b23) (MkPoint n0 n1)) =
  hull_contains_left b01 b23 (MkPoint n0 n1) Proved

theorem seed_law_hull_right :
  IsTrue (contains (hull b01 b23) (MkPoint n3 n2)) =
  hull_contains_right b01 b23 (MkPoint n3 n2) Proved

theorem seed_law_manhattan_symm :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1))
            (manhattan (MkPoint n3 n1) (MkPoint n1 n4)) =
  manhattan_symm (MkPoint n1 n4) (MkPoint n3 n1)

theorem seed_law_manhattan_self :
  Equal Nat (manhattan (MkPoint n3 n4) (MkPoint n3 n4)) Zero =
  manhattan_self (MkPoint n3 n4)
```

## Single-axis membership includes both endpoints

### stdlib/geometry/axis-membership

- spec: `spec/50-stdlib/67-geometry.md` §1.1.
- promise class: durable invariant.
- given: the closed interval `[1,2]` and the inverted interval `[2,1]`.
- expect: `axis_contains 1 2 1 = True`, `axis_contains 1 2 2 = True`,
  `axis_contains 1 2 0 = False`, `axis_contains 1 2 3 = False`,
  and `axis_contains 2 1 2 = False`.
- measured: both inclusive endpoints, both outside sides, and an
  inverted axis without membership at a would-be endpoint.
- claimed: both natural-order comparisons are required for membership.
- gap: five fixed inputs do not prove the generic order conjunction.
- why: deleting either bound admits a below- or above-range point
  which the corresponding direct oracle rejects.

## Box membership checks both coordinates

### stdlib/geometry/box-membership

- spec: `spec/50-stdlib/67-geometry.md` §1.1.
- promise class: durable invariant.
- given: `b02`, `in_both=(1,2)`, and `(3,1)`.
- expect: `contains b02 in_both = True` and
  `contains b02 (3,1) = False`.
- measured: a boundary point succeeds, whereas a point outside only
  the horizontal bound fails with the vertical coordinate inside.
- claimed: a box point needs both axis comparisons.
- gap: these cases do not isolate every possible bound or prove the law.
- why: the negative differs on one coordinate while retaining the other.

## Intersection preserves corners and can be empty

### stdlib/geometry/intersection-corners

- spec: `spec/50-stdlib/67-geometry.md` §1.2.
- promise class: durable invariant.
- given: `b02`, `b13`, and separately disjoint `b01`, `b23`.
- expect: `intersect b02 b13` has corners `(1,1)` and `(2,2)`;
  `intersect b01 b23` has `(2,2)` and `(1,1)`. The disjoint
  intersection contains neither input's nearest corner.
- measured: coordinatewise min/max order, an inverted-corner result
  rather than `Option`, and two corner-membership refusals.
- claimed: `intersect` is total and its points are in both boxes.
- gap: fixed corners do not prove equivalence for arbitrary boxes;
  the generic law obligations do that separately.
- why: a disjoint pair must preserve the inverted representation
  and reject both candidate endpoints.

## One-box-only point remains outside the intersection

### stdlib/geometry/intersection-one-box-only

- spec: `spec/50-stdlib/67-geometry.md` §§1.1–1.3.
- promise class: durable invariant.
- given: `only_first=(0,0)` lies in `b02` but not `b13`.
- expect: `contains b02 only_first = True`,
  `contains b13 only_first = False`, and
  `contains (intersect b02 b13) only_first = False`.
- measured: the three Boolean results on the **same point**.
- claimed: intersection requires membership in both, not either.
- gap: this closed point does not establish the generic theorem.
- why: changing intersection to keep a point from only `b02`
  contradicts the independently pinned False result; the refusal
  below tests the same opposite proposition with the positive client
  unchanged.

## Hull contains both boxes and preserves corner arithmetic

### stdlib/geometry/hull-corners

- spec: `spec/50-stdlib/67-geometry.md` §1.2.
- promise class: durable invariant.
- given: disjoint `b01` and `b23`.
- expect: `hull b01 b23` has corners `(0,0)` and `(3,3)`.
- measured: exact structural corners, not a set of sampled points.
- claimed: the coordinatewise hull bounds both inputs.
- gap: this is one pair; the generic left/right laws separately
  establish membership for every point in either input.
- why: choosing `max` for the lower corner or `min` for the upper
  corner disagrees with the expected result.

## Manhattan distance adds both absolute axis differences

### stdlib/geometry/manhattan-value

- spec: `spec/50-stdlib/67-geometry.md` §1.2.
- promise class: durable invariant.
- given: `(1,4)` and `(3,1)`.
- expect: `manhattan` returns `5` (`|1−3| + |4−1| = 2+3`).
- measured: different nonzero contributions from both axes.
- claimed: the measure is Manhattan distance using saturating `sub`.
- gap: one closed pair does not prove symmetry, self-distance or
  the distance of arbitrary points.
- why: ignoring either axis or using a single signed subtraction
  changes this result.

## Six generic laws instantiated on closed fixtures

### stdlib/geometry/intersection-sound

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `in_both` in `intersect b02 b13`.
- expect: `intersect_sound b02 b13 in_both Proved` yields both
  `IsTrue` proofs in `And`.
- measured: application of the generically checked soundness law.
- claimed: an intersection point belongs to each input.
- gap: one application does not independently prove the law.
- why: separate evidence for both inputs prevents an either-box claim.

### stdlib/geometry/intersection-complete

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `(2,1)` lies in both `b02` and `b13`.
- expect: `intersect_complete b02 b13 (2,1) Proved Proved`
  yields `IsTrue` of its membership in the intersection.
- measured: application with two independently discharged premises.
- claimed: no point belonging to both boxes is dropped.
- gap: the single closed fixture is not a generic proof.
- why: the law requires both premises; a proof-free assumption would
  not check with the original theorem signature.

### stdlib/geometry/hull-left

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `(0,1)` belongs to `b01`.
- expect: `hull_contains_left b01 b23 (0,1) Proved` proves its
  membership in the hull.
- measured: application of the generic left-input law.
- claimed: every point in the left box remains in the hull.
- gap: one point cannot establish the quantifier.
- why: dropping the lower left corner makes the displayed fixture
  fail, while the generic checked law handles all points.

### stdlib/geometry/hull-right

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `(3,2)` belongs to `b23`.
- expect: `hull_contains_right b01 b23 (3,2) Proved` proves its
  membership in the hull.
- measured: application of the generic right-input law.
- claimed: every point in the right box remains in the hull.
- gap: one point cannot establish the quantifier.
- why: dropping the high right corner makes this fixture fail,
  independently of the left-input instance.

### stdlib/geometry/manhattan-symmetry

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `(1,4)` and `(3,1)`.
- expect: `manhattan_symm` inhabits their distance equality
  under reversal.
- measured: specialization of the generically checked theorem.
- claimed: distance is symmetric for all points.
- gap: this one pair cannot prove symmetry generically.
- why: both ordered calls are named in the instantiated goal.

### stdlib/geometry/manhattan-self

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `(3,4)`.
- expect: `manhattan_self` yields `Equal Nat` between its own
  distance and `Zero`.
- measured: specialization of the generically checked theorem.
- claimed: any point has zero distance from itself.
- gap: this instance does not establish the generic statement.
- why: no Euclidean or triangle-inequality law is inferred.

## Required refusals, one at a time

Each following `ken` block is appended **alone** to the same positive
client. A rejection has force only alongside the successfully
checked positive block, against the same scratch module; no
negative is meant to be run without that control. These test
false propositions, not compiler syntax or repository text.

### stdlib/geometry/reject-one-box-only-point

- spec: `spec/50-stdlib/67-geometry.md` §§1.1–1.3.
- promise class: durable invariant.
- given: `only_first` belongs to `b02` but not `b13`.
- expect: the claim that it belongs to `intersect b02 b13` rejects.
- measured: refusal of the opposite of the positive client's
  `only_first_outside_intersection` assertion.
- claimed: intersection does not retain a point in only one box.
- gap: one negative cannot exclude other incorrect intersections.
- why: changing only the expected membership flips the checker
  verdict; it is not a syntactically invalid program.

```ken
-- Reject: point in only b02 cannot be in their intersection.
theorem reject_one_box_only_point :
  IsTrue (contains (intersect b02 b13) only_first) = Proved
```

### stdlib/geometry/reject-fabricated-other-premise

- spec: `spec/50-stdlib/67-geometry.md` §1.3.
- promise class: durable invariant.
- given: `only_first` is in `b02` but not `b13`.
- expect: attempting `intersect_complete` with a fabricated proof
  of `contains b13 only_first` rejects.
- measured: the false second premise, not a result computed by
  intersection.
- claimed: the completeness law cannot prove membership from just
  one box's premise.
- gap: this is a closed argument check; the generic theorem was
  established separately by D0.
- why: a law application may not launder a false `IsTrue` premise.

```ken
-- Reject: Proved cannot inhabit the missing b13 membership premise.
theorem reject_fabricated_other_premise :
  IsTrue (contains (intersect b02 b13) only_first) =
  intersect_complete b02 b13 only_first Proved Proved
```

### stdlib/geometry/reject-point-in-empty-meet

- spec: `spec/50-stdlib/67-geometry.md` §1.1–1.2.
- promise class: durable invariant.
- given: the inverted bounds of `intersect b01 b23`.
- expect: its input corner `(1,1)` is not a member.
- measured: a false `IsTrue` goal about the empty result.
- claimed: inverted bounds contain no point.
- gap: one point does not prove emptiness generally.
- why: the same corner is positively checked as excluded above.

```ken
-- Reject: an empty intersection contains no input corner.
theorem reject_point_in_empty_meet :
  IsTrue (contains (intersect b01 b23) (MkPoint n1 n1)) = Proved
```

### stdlib/geometry/reject-wrong-hull-corners

- spec: `spec/50-stdlib/67-geometry.md` §1.2.
- promise class: durable invariant.
- given: disjoint `b01` and `b23`.
- expect: a hull with lower corner `(1,1)` rejects.
- measured: structural `Eq Box` disagreement with the direct
  `hull_corners` oracle.
- claimed: hull uses each coordinate's minimum for the lower corner.
- gap: this one changed corner does not prove the general law.
- why: `Eq Box` is about corners, not just sampled membership.

```ken
-- Reject: wrong lower corner, unchanged upper corner.
theorem reject_wrong_hull_corners :
  Eq Box (hull b01 b23)
    (MkBox (MkPoint n1 n1) (MkPoint n3 n3)) = Refl
```

### stdlib/geometry/reject-wrong-distance

- spec: `spec/50-stdlib/67-geometry.md` §1.2.
- promise class: durable invariant.
- given: `(1,4)` and `(3,1)`.
- expect: asserting distance `4` instead of `5` rejects.
- measured: a one-value change against the independent numeric oracle.
- claimed: both unequal coordinate differences contribute to `5`.
- gap: this says nothing about triangle inequality or all point pairs.
- why: the positive distance oracle holds every other input fixed.

```ken
-- Reject: one less than the five required by two nonzero axes.
theorem reject_wrong_distance :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1)) n4 = Proved
```
