# Manhattan distance bounded by a containing box

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §10.
Frame: `docs/program/issues/SPEC-GEOMETRY-MANHATTAN-BOX-DIAMETER.md`.

This seed is authored against candidate child
`4132a9c0391163d72f32e8fc3b7012261a3a6e63`, which contains the §10
contract and catalog/test delivery. It is authored but unexecuted. The
closed Ken fences below were not run in this author turn.

The box `(1,1)–(4,3)` has corner distance `5`. Its contained points
`(1,3)` and `(4,1)` are also distance `5` apart, so the public bound
is tight. Moving the first point to `(0,3)` puts it outside the same
box and makes its distance to `(4,1)` equal `6`. The finite cases
exercise the public law and its membership premise; they do not prove
the generic bound.

## Shared fixtures and accepted client

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, contains, manhattan,
   manhattan_box_diameter)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

const box_lo : Point = MkPoint one one
const box_hi : Point = MkPoint four three
const sample_box : Box = MkBox box_lo box_hi
const contained_p : Point = MkPoint one three
const contained_q : Point = MkPoint four one
const outside_p : Point = MkPoint zero three

theorem contained_p_is_in_box :
  IsTrue (contains sample_box contained_p) = Proved

theorem contained_q_is_in_box :
  IsTrue (contains sample_box contained_q) = Proved

theorem box_corner_distance_is_five :
  Equal Nat (manhattan box_lo box_hi) five = Proved

theorem contained_pair_distance_is_five :
  Equal Nat (manhattan contained_p contained_q) five = Proved

theorem contained_pair_bound_is_true :
  Equal Bool
    (leq_nat (manhattan contained_p contained_q)
             (manhattan box_lo box_hi))
    True = Proved

theorem contained_pair_satisfies_box_diameter :
  IsTrue
    (leq_nat (manhattan contained_p contained_q)
             (manhattan box_lo box_hi)) =
  manhattan_box_diameter
    box_lo box_hi contained_p contained_q
    contained_p_is_in_box contained_q_is_in_box

theorem outside_p_is_not_in_box :
  Equal Bool (contains sample_box outside_p) False = Proved

theorem outside_pair_distance_is_six :
  Equal Nat (manhattan outside_p contained_q) six = Proved

theorem outside_pair_bound_is_false :
  Equal Bool
    (leq_nat (manhattan outside_p contained_q)
             (manhattan box_lo box_hi))
    False = Proved
```

### stdlib/geometry/manhattan-box-diameter-tight-pair

- spec: §10.
- promise class: durable invariant.
- given: the closed box `(1,1)–(4,3)` and contained pair
  `(1,3),(4,1)`.
- expect: both distances are `5`; the public law establishes the
  corresponding `leq_nat` proposition.
- measured: when run, separate `Proved` results for both memberships,
  both distances and the closed bound, plus one application of
  `manhattan_box_diameter`.
- claimed: points contained in the box are no farther apart than its
  stored corners.
- gap: this tight closed pair does not establish the generic law.
- why: the contained pair attains the corner distance, so a strictly
  smaller claimed bound is distinguishable.

## Isolated refusals

For each refusal, start from a fresh copy of the complete accepted
client above and append only its fence. Expect the base alone to accept
and the appended input to reject at its own theorem, not at an import
or an earlier control.

### stdlib/geometry/reject-outside-pair-bound

- spec: §10.
- promise class: durable invariant.
- given: `outside_p=(0,3)` is outside the box, `contained_q=(4,1)`
  is inside, and their distance `6` exceeds the corner distance `5`.
- expect: refusal of the false bound for this pair.
- measured: when run, the accepted prefix checks non-membership,
  distance `6` and the false comparison; this fence claims the bound
  without the law's membership premise.
- claimed: the closed upper bound does not extend to a point outside
  the box.
- gap: this closed counterexample does not characterize all invalid
  pairs.
- why: its false `6 ≤ 5` comparison has an independent value control.

```ken
theorem reject_outside_pair_bound :
  IsTrue
    (leq_nat (manhattan outside_p contained_q)
             (manhattan box_lo box_hi)) = Proved
```

### stdlib/geometry/reject-law-with-outside-premise

- spec: §10.
- promise class: durable invariant.
- given: the same outside pair and the accepted membership proof for
  `contained_q`.
- expect: refusal when `Proved` is supplied as the outside point's
  false membership premise to `manhattan_box_diameter`.
- measured: the accepted prefix states that `contains sample_box
  outside_p` is `False`; when run, the appended law application must
  fail at its first premise.
- claimed: the law cannot be applied without a proof that both points
  are contained in the stated box.
- gap: this one false premise does not characterize every misuse of
  the law.
- why: the two distance controls show why an unconditional bound is
  false, while the application isolates the missing membership proof.

```ken
theorem reject_law_with_outside_premise :
  IsTrue
    (leq_nat (manhattan outside_p contained_q)
             (manhattan box_lo box_hi)) =
  manhattan_box_diameter
    box_lo box_hi outside_p contained_q
    Proved contained_q_is_in_box
```

### stdlib/geometry/reject-bound-below-tight-pair

- spec: §10.
- promise class: durable invariant.
- given: the accepted contained pair, whose distance and corner
  distance are both `5`.
- expect: refusal of a bound by `4`.
- measured: when run, the accepted prefix proves the pair distance is
  `5` and the public bound holds at `5`.
- claimed: the corner-distance bound is tight for this pair.
- gap: this closed pair does not prove the generic law or rule out
  every weaker bound on other boxes.
- why: the false `5 ≤ 4` comparison is isolated from membership and
  theorem availability.

```ken
theorem reject_bound_below_tight_pair :
  IsTrue (leq_nat (manhattan contained_p contained_q) four) = Proved
```

## Evidence boundary

This seed is authored but unexecuted; the package-owned target was not
run in this author turn. No official conformance-runner execution, CI,
publication, landing, or landed-trust result is claimed.
