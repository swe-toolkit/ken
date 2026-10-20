# Rational interval inclusion and isotonicity

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §3.

These cases are authored source-shaped closed behavioral oracles for
§3's endpoint inclusion operation and generic laws. Each case has
promise class **durable invariant**: it states behavior fixed by the
spec, not current catalog status. A closed application does not
establish a generic law's quantifiers. Here, `measured` describes the
intended oracle, not a run report. No execution of this seed is
claimed here.

`MkRational n k` denotes `n/(Suc k)`. For soundness,
`[1,3]` lies inside `[0,4]`, and `2` belongs to both. For each
isotonicity law, `[1,2]` lies inside `[0,3]` and `[2,3]` lies
inside `[1,4]`. Their sums are `[3,5]` and `[1,7]`; their products
are `[2,6]` and `[0,12]`. The lower and upper endpoint checks both
hold for each result pair.

The refusal controls use `wide = [1,4]`,
`narrower_by_lower = [2,4]`, and
`narrower_by_upper = [1,3]`. Each narrower interval lies inside
`wide`. The reverse inclusion fails once at a time: only the lower
comparison fails for `wide` in `narrower_by_lower`; only the upper
comparison fails for `wide` in `narrower_by_upper`.

The closed client below checks the nested premises and results
beside each law application. It also checks self-inclusion and both
orientations of the narrower-interval controls. These controls are
part of the intended client; they are not execution evidence.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_add, interval_mul,
   interval_inside, interval_inside_sound, interval_add_inside,
   interval_mul_inside)

const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero

const sound_inner : Interval = MkInterval one three
const sound_outer : Interval = MkInterval zero four
const inner_i : Interval = MkInterval one two
const outer_i : Interval = MkInterval zero three
const inner_j : Interval = MkInterval two three
const outer_j : Interval = MkInterval one four
const wide : Interval = MkInterval one four
const narrower_by_lower : Interval = MkInterval two four
const narrower_by_upper : Interval = MkInterval one three

theorem sound_nested_pair :
  Equal Bool (interval_inside sound_inner sound_outer) True = Proved

theorem sound_inner_member :
  Equal Bool (interval_contains sound_inner two) True = Proved

theorem sound_outer_member :
  Equal Bool (interval_contains sound_outer two) True = Proved

theorem soundness_law_instance :
  IsTrue (interval_contains sound_outer two) =
  interval_inside_sound sound_inner sound_outer two Proved Proved

theorem add_left_input_inside :
  Equal Bool (interval_inside inner_i outer_i) True = Proved

theorem add_right_input_inside :
  Equal Bool (interval_inside inner_j outer_j) True = Proved

theorem add_result_inside :
  Equal Bool
    (interval_inside (interval_add inner_i inner_j) (interval_add outer_i outer_j))
    True = Proved

theorem addition_isotonicity_instance :
  IsTrue (interval_inside (interval_add inner_i inner_j) (interval_add outer_i outer_j)) =
  interval_add_inside inner_i outer_i inner_j outer_j Proved Proved

theorem mul_left_input_inside :
  Equal Bool (interval_inside inner_i outer_i) True = Proved

theorem mul_right_input_inside :
  Equal Bool (interval_inside inner_j outer_j) True = Proved

theorem mul_result_inside :
  Equal Bool
    (interval_inside (interval_mul inner_i inner_j) (interval_mul outer_i outer_j))
    True = Proved

theorem multiplication_isotonicity_instance :
  IsTrue (interval_inside (interval_mul inner_i inner_j) (interval_mul outer_i outer_j)) =
  interval_mul_inside inner_i outer_i inner_j outer_j Proved Proved

theorem self_inclusion_control :
  Equal Bool (interval_inside sound_inner sound_inner) True = Proved

theorem narrower_by_lower_inside_wide :
  Equal Bool (interval_inside narrower_by_lower wide) True = Proved

theorem wider_not_inside_narrower_by_lower :
  Equal Bool (interval_inside wide narrower_by_lower) False = Proved

theorem narrower_by_upper_inside_wide :
  Equal Bool (interval_inside narrower_by_upper wide) True = Proved

theorem wider_not_inside_narrower_by_upper :
  Equal Bool (interval_inside wide narrower_by_upper) False = Proved
```

## Soundness on a closed nested pair

### stdlib/numerical/interval-inside-soundness

- spec: `spec/50-stdlib/68-numerical.md` §3.2.
- promise class: durable invariant.
- given: `[1,3]` inside `[0,4]`, with member `2` in the inner
  interval.
- expect: the closed `interval_inside_sound` application establishes
  membership of `2` in the outer interval.
- measured: the inclusion and both memberships are checked directly;
  the law application has those same closed premises and conclusion.
- claimed: an included interval's members belong to the containing
  interval.
- gap: this finite instance does not establish the generic theorem.
- why: the positive controls establish each premise independently,
  while the law application reaches the public theorem.

## Addition is isotonic on closed nested pairs

### stdlib/numerical/interval-add-inside

- spec: `spec/50-stdlib/68-numerical.md` §3.2.
- promise class: durable invariant.
- given: `[1,2]` inside `[0,3]` and `[2,3]` inside `[1,4]`.
- expect: the sum `[3,5]` is inside `[1,7]`.
- measured: both input inclusions, the result inclusion, and a closed
  `interval_add_inside` application.
- claimed: addition preserves inclusion in both interval arguments.
- gap: this finite instance does not establish the generic law.
- why: the two input controls establish both premises, and the direct
  result check independently checks the law's conclusion.

## Multiplication is isotonic on closed nested pairs

### stdlib/numerical/interval-mul-inside

- spec: `spec/50-stdlib/68-numerical.md` §3.2.
- promise class: durable invariant.
- given: `[1,2]` inside `[0,3]` and `[2,3]` inside `[1,4]`.
- expect: the product `[2,6]` is inside `[0,12]`.
- measured: both input inclusions, the result inclusion, and a closed
  `interval_mul_inside` application.
- claimed: multiplication preserves inclusion in both interval
  arguments over non-negative rational endpoints.
- gap: this finite instance does not establish the generic law.
- why: the inputs are non-negative; each input-inclusion premise and
  the result are directly checked.

## Self-inclusion is a positive control

### stdlib/numerical/interval-inside-self-control

- spec: `spec/50-stdlib/68-numerical.md` §3.2.
- promise class: durable invariant.
- given: `[1,3]` as both arguments to `interval_inside`.
- expect: self-inclusion is `True`.
- measured: the direct Boolean result in the closed client.
- claimed: the inclusion operation accepts an interval as included in
  itself.
- gap: one closed interval does not establish `interval_inside_refl`
  for every interval.
- why: this positive control shares the operation and endpoints used
  by the refused reverse-inclusion cases.

## Refusals, appended one at a time

Each block is appended alone to the unchanged positive client. The
client directly checks that the narrower interval lies inside `wide`
and that the reverse Boolean result is `False`; a refusal is not
standalone evidence if the positive client does not check.

### stdlib/numerical/reject-wide-inside-narrower-by-lower

- spec: `spec/50-stdlib/68-numerical.md` §3.1.
- promise class: durable invariant.
- given: `wide = [1,4]` and `narrower_by_lower = [2,4]`.
- expect: claiming `wide` lies inside `narrower_by_lower` is refused.
- measured: only the lower comparison fails; the same client checks
  the reverse inclusion as `True` and the target claim as `False`.
- claimed: inclusion requires the containing interval's lower
  endpoint to be no greater than the inner interval's lower endpoint.
- gap: this fixture does not establish the other endpoint comparison;
  the next refusal isolates it.
- why: the upper endpoints are equal, so rejection is attributable to
  the lower comparison.

```ken
-- Reject: the target's lower endpoint is greater than wide's.
theorem reject_wide_inside_narrower_by_lower :
  IsTrue (interval_inside wide narrower_by_lower) = Proved
```

### stdlib/numerical/reject-wide-inside-narrower-by-upper

- spec: `spec/50-stdlib/68-numerical.md` §3.1.
- promise class: durable invariant.
- given: `wide = [1,4]` and `narrower_by_upper = [1,3]`.
- expect: claiming `wide` lies inside `narrower_by_upper` is refused.
- measured: only the upper comparison fails; the same client checks
  the reverse inclusion as `True` and the target claim as `False`.
- claimed: inclusion requires the inner interval's upper endpoint to
  be no greater than the containing interval's upper endpoint.
- gap: this fixture does not establish the other endpoint comparison;
  the preceding refusal isolates it.
- why: the lower endpoints are equal, so rejection is attributable to
  the upper comparison.

```ken
-- Reject: wide's upper endpoint exceeds the target's.
theorem reject_wide_inside_narrower_by_upper :
  IsTrue (interval_inside wide narrower_by_upper) = Proved
```
