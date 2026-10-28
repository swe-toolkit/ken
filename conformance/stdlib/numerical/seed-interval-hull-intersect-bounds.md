# Rational interval hull and intersection bounds

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §7.
Ruling: Architect AC-0 `evt_888gf2j697ab`.

This seed gives closed instances of both §7 laws over nondegenerate
intervals, each with distinct lower and upper rational endpoints. `low` and
`high` are strictly inside `outer`; `narrow` is strictly inside both
intersection operands. The seed computes each primary premise and
conclusion independently with `Proved`, checks the closed endpoints with
`Refl`, and applies both laws to an inverted interval. The refused hull claim
is isolated after the accepted client and paired with a same-shape accepted
control and independent `False` value. These finite cases illustrate the
generic laws; they do not prove the universal quantifiers.

## Accepted closed client

Accept this entire client before testing the isolated refusal. The import list
does not import `IsTrue` from `Core.Classes.LawfulClasses`; the AC-0 scratch
client checked the same import as ambiguous, and `IsTrue` is already
available to the client.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_hull,
    interval_intersect,
    interval_inside,
    interval_hull_least,
    interval_intersect_greatest)

const zero : Rational = MkRational Zero Zero

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const one : Rational = MkRational (Suc Zero) Zero

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero

const low : Interval = MkInterval half one

const high : Interval = MkInterval two three

const outer : Interval = MkInterval zero four

const wide_left : Interval = MkInterval zero three

const wide_right : Interval = MkInterval one four

const narrow : Interval = MkInterval two three

const inverted : Interval = MkInterval four zero

theorem low_inside_outer : Equal Bool (interval_inside low outer) True = Proved

theorem low_inside_low : Equal Bool (interval_inside low low) True = Proved

theorem high_inside_outer : Equal Bool (interval_inside high outer) True = Proved

theorem hull_endpoints : Equal Interval (interval_hull low high) (MkInterval half three) = Refl

theorem hull_inside_outer_value
    : Equal Bool (interval_inside (interval_hull low high) outer) True =
  Proved

theorem outer_not_inside_hull
    : Equal Bool (interval_inside outer (interval_hull low high)) False =
  Proved

theorem hull_inside_outer_by_law : IsTrue (interval_inside (interval_hull low high) outer) =
  interval_hull_least low high outer Proved Proved

theorem narrow_inside_wide_left : Equal Bool (interval_inside narrow wide_left) True = Proved

theorem narrow_inside_wide_right : Equal Bool (interval_inside narrow wide_right) True = Proved

theorem intersect_endpoints
    : Equal Interval (interval_intersect wide_left wide_right) (MkInterval one three) =
  Refl

theorem narrow_inside_intersect_value
    : Equal Bool (interval_inside narrow (interval_intersect wide_left wide_right)) True =
  Proved

theorem intersect_not_inside_narrow
    : Equal Bool (interval_inside (interval_intersect wide_left wide_right) narrow) False =
  Proved

theorem narrow_inside_intersect_by_law
    : IsTrue (interval_inside narrow (interval_intersect wide_left wide_right)) =
  interval_intersect_greatest narrow wide_left wide_right Proved Proved

theorem inverted_inside_wide_left : Equal Bool (interval_inside inverted wide_left) True =
  Proved

theorem inverted_inside_wide_right : Equal Bool (interval_inside inverted wide_right) True =
  Proved

theorem inverted_inside_intersect_value
    : Equal Bool (interval_inside inverted (interval_intersect wide_left wide_right)) True =
  Proved

theorem inverted_inside_intersect_by_law
    : IsTrue (interval_inside inverted (interval_intersect wide_left wide_right)) =
  interval_intersect_greatest inverted wide_left wide_right Proved Proved

theorem inverted_low_inside_outer : Equal Bool (interval_inside inverted outer) True = Proved

theorem inverted_hull_inside_outer_value
    : Equal Bool (interval_inside (interval_hull inverted low) outer) True =
  Proved

theorem inverted_hull_inside_outer_by_law
    : IsTrue (interval_inside (interval_hull inverted low) outer) =
  interval_hull_least inverted low outer Proved Proved

theorem same_shape_hull_control : IsTrue (interval_inside (interval_hull low low) low) =
  interval_hull_least low low low Proved Proved

theorem high_not_inside_low : Equal Bool (interval_inside high low) False = Proved

theorem hull_not_inside_low : Equal Bool (interval_inside (interval_hull low high) low) False =
  Proved
```

### `stdlib/numerical/interval-hull-least-closed-bound`

- spec: §7.1, `interval_hull_least`.
- promise class: durable invariant.
- given: `low` and `high` are each inside `outer`.
- expect: the law proves that `interval_hull low high` is inside `outer`.
  The direct `Proved` result independently computes the conclusion.
- measured: both inclusion premises, the endpoint formula, and the result
  inclusion are checked independently; `outer_not_inside_hull` shows that
  `outer` is strictly wider than the hull.
- claimed: a common endpoint-wise outer bound contains the hull of its two
  operand intervals.
- gap: these endpoints do not prove the generic law for all intervals.
- why: each interval is nondegenerate; the outer bound's endpoints are
  distinct from the hull's, and `Refl` fixes the hull result independently.

### `stdlib/numerical/interval-intersect-greatest-closed-bound`

- spec: §7.1, `interval_intersect_greatest`.
- promise class: durable invariant.
- given: `narrow` is inside `wide_left` and `wide_right`.
- expect: the law proves that `narrow` is inside their intersection. The
  direct `Proved` result independently computes the conclusion.
- measured: both premises, the endpoint formula, and the result inclusion
  are checked independently; `intersect_not_inside_narrow` shows the
  intersection is strictly wider than `narrow`.
- claimed: a common endpoint-wise inner bound lies inside the intersection
  of its two operand intervals.
- gap: these endpoints do not prove the generic law for all intervals.
- why: each interval is nondegenerate, and the endpoint formula is
  independently checked with `Refl`.

### `stdlib/numerical/inverted-intervals-use-the-same-endpoint-laws`

- spec: §7.1, including inverted intervals.
- promise class: durable invariant.
- given: `inverted = [4,0]` is used as a hull operand and as the common
  inner interval for an intersection.
- expect: each law application checks; separate `Proved` values check the
  two intersection premises, the intersection result, the hull premise, and
  the hull result.
- measured: the stored endpoint comparisons of the inverted interval and
  the two law results; no nonempty-member premise is used.
- claimed: the laws apply to inverted intervals under the same
  endpoint-value relation.
- gap: one inverted value does not characterize all empty intervals.
- why: the spec defines `interval_inside` from endpoint comparisons and
  requires no interval-validity premise.

## Isolated refused hull claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. The accepted control fixes the first hull operand and bound at
`low`; the refused claim changes only the second hull operand from `low` to
`high`. `high_not_inside_low` independently computes the false premise, and
`hull_not_inside_low` computes the false conclusion. Require refusal at
`seed_neg`, not at an import or earlier declaration.

### `stdlib/numerical/reject-hull-inside-strict-operand`

- spec: §7.1, which does not state that a hull lies inside either operand.
- promise class: durable invariant.
- given: the complete accepted client, including the same-shape hull control
  and independent false values, followed by `seed_neg` alone.
- expect: reject the claim that the hull of `low` and `high` is inside `low`.
- measured: the same-shape control accepts; `high_not_inside_low` and
  `hull_not_inside_low` independently evaluate to `False`.
- claimed: the hull law cannot be reversed to conclude that a hull lies
  inside one operand when the other operand is outside it.
- gap: this counterexample does not characterize all invalid hull bounds.
- why: the accepted control and refusal share the same law/result form,
  first operand, and bound. Changing only the second operand makes the
  distinction attributable to the independently false inclusion of `high` in
  `low`.

```ken
theorem seed_neg : IsTrue (interval_inside (interval_hull low high) low) = Proved
```

## Evidence boundary

Architect AC-0's D0 on base `1363b09b4` checked the two generic law bodies and
an independent generic client, with no new `Axiom` and equal scratch
`trusted_base()` GlobalId sets (114 → 114). Its closed client applied both
laws to rational-interval fixtures, computed the premises and results
independently, checked an inverted interval, and rejected the false hull
claim after the accepted client. The D0 patch digest is
`e448b72cf4af08bdbcce99996842f94de3875e0c158a7719b2c0dd40285cebaa`.
Those were Architect-reported local scratch checks, not execution of this
exact seed or shipped §7 exports. The §7 catalog laws were absent on the
spec-and-seed candidate's base. This separate Foundation candidate adds
both checked exports and runs the complete accepted client and isolated
false hull claim in the package-owned
`cat_numeric_interval_hull_intersect_bounds` test. Its exact trusted
GlobalId comparison and targeted RationalInterval ambient row are
candidate-local measurements, not a full-corpus census or landed trust.
This package test is not official conformance-runner execution, CI,
publication or landing.
