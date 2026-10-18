# Rational interval hull and intersection

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §2.

These cases are authored source-shaped closed behavioral oracles for the
prospective `Algorithm.Numeric.RationalInterval` extension. They instantiate
exactly the four generic §2 laws: `interval_hull_contains_left`,
`interval_hull_contains_right`, `interval_intersect_sound`, and
`interval_intersect_complete`. Closed instances do not prove the generic
quantifiers. The §2 catalog extension and the required Rational min/max
exports are not shipped by this seed.

The D0 checked the four generic laws and a separate closed client in scratch.
Its closed client checked 16 positive assertions; each of four negative clients
rejected when appended alone. These are Architect-reported scratch results,
not execution of this seed against a shipped module, catalog delivery, a Cargo
test, CI, or a `trusted_base()` measurement. This seed does not assert
structural commutativity, idempotence, or associativity.

## Shared fixtures and positive assertions

`MkRational n k` represents `n/(Suc k)`. The disjoint inputs are
`low = [0, 1/2]` and `high = [2, 3]`. Their hull is `[0, 3]`, which contains
`1/2` and `2` from the respective inputs, the gap point `1`, and `3`; it
excludes `4`. Their intersection is the inverted interval `[2, 1/2]`. For an
overlap, `left = [1/2, 2]` and `right = [1, 3]` intersect to `[1, 2]`.
The common points `1` and `3/2` belong to the intersection, while `1/2` does
not.

The closed client below instantiates each of the four public laws. Its direct
endpoint and membership assertions provide independent controls for those
applications. The endpoint values follow §2.1's comparison-selected formulas;
no expected endpoint is computed by calling `interval_hull` or
`interval_intersect`.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_hull,
   interval_intersect, interval_hull_contains_left,
   interval_hull_contains_right, interval_intersect_sound,
   interval_intersect_complete)

const zero : Rational = MkRational Zero Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const three_halves : Rational =
  MkRational (Suc (Suc (Suc Zero))) (Suc Zero)

const low : Interval = MkInterval zero half
const high : Interval = MkInterval two three
const left : Interval = MkInterval half two
const right : Interval = MkInterval one three
const disjoint_hull : Interval = interval_hull low high
const disjoint_intersection : Interval = interval_intersect low high
const overlap_intersection : Interval = interval_intersect left right

theorem low_half_member :
  Equal Bool (interval_contains low half) True = Proved

theorem high_two_member :
  Equal Bool (interval_contains high two) True = Proved

theorem hull_endpoints :
  Equal Interval disjoint_hull (MkInterval zero three) = Refl

theorem hull_contains_left_law :
  IsTrue (interval_contains disjoint_hull half) =
  interval_hull_contains_left low high half Proved

theorem hull_contains_right_law :
  IsTrue (interval_contains disjoint_hull two) =
  interval_hull_contains_right low high two Proved

theorem hull_contains_gap :
  Equal Bool (interval_contains disjoint_hull one) True = Proved

theorem hull_contains_upper_endpoint :
  Equal Bool (interval_contains disjoint_hull three) True = Proved

theorem hull_excludes_four :
  Equal Bool (interval_contains disjoint_hull four) False = Proved

theorem disjoint_intersection_endpoints :
  Equal Interval disjoint_intersection (MkInterval two half) = Refl

theorem disjoint_intersection_excludes_left_endpoint :
  Equal Bool (interval_contains disjoint_intersection half) False = Proved

theorem disjoint_intersection_excludes_right_endpoint :
  Equal Bool (interval_contains disjoint_intersection two) False = Proved

theorem overlap_intersection_endpoints :
  Equal Interval overlap_intersection (MkInterval one two) = Refl

theorem overlap_common_one :
  Equal Bool (interval_contains overlap_intersection one) True = Proved

theorem overlap_common_three_halves :
  Equal Bool (interval_contains overlap_intersection three_halves) True = Proved

theorem left_three_halves_member :
  Equal Bool (interval_contains left three_halves) True = Proved

theorem right_three_halves_member :
  Equal Bool (interval_contains right three_halves) True = Proved

theorem intersection_sound_law :
  And (IsTrue (interval_contains left three_halves))
      (IsTrue (interval_contains right three_halves)) =
  interval_intersect_sound left right three_halves Proved

theorem left_one_member :
  Equal Bool (interval_contains left one) True = Proved

theorem right_one_member :
  Equal Bool (interval_contains right one) True = Proved

theorem intersection_complete_law :
  IsTrue (interval_contains overlap_intersection one) =
  interval_intersect_complete left right one Proved Proved

theorem overlap_excludes_half :
  Equal Bool (interval_contains overlap_intersection half) False = Proved
```

## Closed instances of the four laws

### stdlib/numerical/interval-hull-left

- spec: `spec/50-stdlib/68-numerical.md` §2.2.
- promise class: durable invariant.
- given: `low = [0,1/2]`, `high = [2,3]`, and the member `1/2` of `low`.
- expect: the closed application of `interval_hull_contains_left` proves that
  `1/2` belongs to `interval_hull low high`.
- measured: the premise and the result membership are also checked directly.
- claimed: the left input's member is contained in the hull.
- gap: this instance does not prove the theorem for arbitrary intervals or
  members.
- why: a direct expected hull and endpoint assertion independently fixes the
  result used by the theorem application.

### stdlib/numerical/interval-hull-right

- spec: `spec/50-stdlib/68-numerical.md` §2.2.
- promise class: durable invariant.
- given: `low = [0,1/2]`, `high = [2,3]`, and the member `2` of `high`.
- expect: the closed application of `interval_hull_contains_right` proves that
  `2` belongs to `interval_hull low high`.
- measured: the premise and the result membership are also checked directly.
- claimed: the right input's member is contained in the hull.
- gap: this instance does not prove the theorem for arbitrary intervals or
  members.
- why: the chosen point is the inclusive lower endpoint of the right input.

### stdlib/numerical/interval-intersect-sound

- spec: `spec/50-stdlib/68-numerical.md` §2.2.
- promise class: durable invariant.
- given: the overlap `left = [1/2,2]`, `right = [1,3]`, and common member
  `3/2`.
- expect: the closed application of `interval_intersect_sound` yields
  membership of `3/2` in both input intervals.
- measured: both input memberships and membership in the independently stated
  `[1,2]` intersection are checked directly.
- claimed: every member of the result is a member of each operand.
- gap: the closed application does not establish the generic implication.
- why: the result member and both operand controls are non-boundary values.

### stdlib/numerical/interval-intersect-complete

- spec: `spec/50-stdlib/68-numerical.md` §2.2.
- promise class: durable invariant.
- given: the same overlap and common member `1`.
- expect: the closed application of `interval_intersect_complete` proves that
  `1` belongs to the intersection.
- measured: both input memberships and the result membership are checked
  directly.
- claimed: every common member belongs to the result.
- gap: this instance does not establish completeness for arbitrary intervals
  or members.
- why: `1` is the inclusive lower endpoint of the right input and lies in the
  independently stated `[1,2]` result.

## Disjoint intersection represents the empty result

- spec: `spec/50-stdlib/68-numerical.md` §§1.1, 2.1.
- promise class: durable invariant.
- given: disjoint `[0,1/2]` and `[2,3]`.
- expect: `interval_intersect low high` has inverted endpoints `[2,1/2]`; its
  would-be endpoints `1/2` and `2` are both excluded.
- measured: structural endpoint identity and both endpoint non-memberships.
- claimed: intersection is total and represents disjointness with the ordinary
  inverted-bound empty interval, not an `Option` or a special error.
- gap: finite values do not prove that every disjoint pair yields an empty
  interval.
- why: both boundary controls distinguish this empty result from an inclusive
  interval.

## Refusals, appended one at a time

Each negative block is appended alone to the unchanged positive client. The
D0's corresponding scratch negatives rejected; these cases are not claims of
execution against a shipped module. The positive hull or intersection controls
above establish the expected result for the same fixtures.

### stdlib/numerical/reject-hull-contains-outside-four

- spec: `spec/50-stdlib/68-numerical.md` §2.1.
- promise class: durable invariant.
- given: the hull `[0,3]` of the disjoint inputs and `4` above its upper bound.
- expect: claiming that `4` belongs to the hull rejects.
- measured: the positive client checks `4` is excluded and `3` is included.
- claimed: hull enclosure does not extend beyond the endpoint formula.
- gap: one outside value does not characterize all non-members.
- why: the positive endpoint control on the same hull distinguishes the false
  claim from a parsing or constructor failure.

```ken
-- Reject: four is above the hull's upper endpoint three.
theorem reject_hull_contains_four :
  IsTrue (interval_contains disjoint_hull four) = Proved
```

### stdlib/numerical/reject-disjoint-intersection-contains-half

- spec: `spec/50-stdlib/68-numerical.md` §§1.1, 2.1.
- promise class: durable invariant.
- given: the inverted intersection `[2,1/2]` of the disjoint inputs.
- expect: claiming that its endpoint `1/2` is a member rejects.
- measured: the positive client checks both would-be endpoints are excluded;
  `interval_intersect_complete` has a separate common-member control on the
  overlapping inputs.
- claimed: disjoint intersection has no members.
- gap: this one empty result does not characterize all disjoint intervals.
- why: the same endpoint is included in `low` but excluded from the inverted
  result.

```ken
-- Reject: the inverted interval has no member at its upper endpoint.
theorem reject_disjoint_intersection_contains_half :
  IsTrue (interval_contains disjoint_intersection half) = Proved
```

### stdlib/numerical/reject-overlap-intersection-contains-half

- spec: `spec/50-stdlib/68-numerical.md` §2.1–2.2.
- promise class: durable invariant.
- given: `[1/2,2] ∩ [1,3] = [1,2]`; `1/2` belongs only to the left input.
- expect: claiming that `1/2` belongs to the intersection rejects.
- measured: the positive client checks the common points `1` and `3/2` are
  included, and `1/2` is excluded.
- claimed: intersection admits common members, not every member of either
  operand.
- gap: one non-common point does not establish the generic soundness law.
- why: a common-point control on the same overlap prevents a vacuous rejection
  from an unusable intersection fixture.

```ken
-- Reject: one-half is not a member of the right input or the intersection.
theorem reject_overlap_intersection_contains_half :
  IsTrue (interval_contains overlap_intersection half) = Proved
```
