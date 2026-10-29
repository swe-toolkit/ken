# Rational interval hull and intersection absorption

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §8.
Ruling: Architect AC-0; see spec §8.

This seed checks four public directions of §8's two mutual-inclusion
absorption pairs on overlapping, nondegenerate intervals. It independently
checks the closed Boolean results with `Proved`, and applies each public law
to those same inputs. The intervals are not inside one another. The moved-
shared-argument claim is isolated after a complete accepted client and a
same-shape accepted control, with an independent `False` value. These closed
instances illustrate the generic laws; they do not prove their universal
quantifiers.

## Accepted closed client

Accept this entire client before testing the isolated refusal. The imports
name all public operations and laws used below. `IsTrue` is already available
to the client.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_hull,
    interval_intersect,
    interval_inside,
    interval_intersect_inside_left,
    interval_inside_intersect_hull_left,
    interval_inside_hull_left,
    interval_hull_intersect_inside_left)

const zero : Rational = MkRational Zero Zero

const one : Rational = MkRational (Suc Zero) Zero

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const left : Interval = MkInterval zero two

const right : Interval = MkInterval one three

const hull_value : Interval = MkInterval zero three

const intersect_value : Interval = MkInterval one two

theorem neither_inside_left_right
    : Equal Bool (interval_inside left right) False = Proved

theorem neither_inside_right_left
    : Equal Bool (interval_inside right left) False = Proved

theorem hull_endpoints
    : Equal Interval (interval_hull left right) hull_value = Refl

theorem intersect_endpoints
    : Equal Interval (interval_intersect left hull_value) left = Refl

theorem left_intersect_hull_value
    : Equal Bool (interval_inside (interval_intersect left hull_value) left) True =
  Proved

theorem left_inside_intersect_hull_value
    : Equal Bool (interval_inside left (interval_intersect left hull_value)) True =
  Proved

theorem left_inside_hull_intersect_value
    : Equal Bool (interval_inside left (interval_hull left (interval_intersect left right))) True =
  Proved

theorem hull_intersect_inside_left_value
    : Equal Bool (interval_inside (interval_hull left (interval_intersect left right)) left) True =
  Proved

theorem intersect_inside_left_by_law
    : IsTrue (interval_inside (interval_intersect left (interval_hull left right)) left) =
  interval_intersect_inside_left left (interval_hull left right)

theorem left_inside_intersect_hull_by_law
    : IsTrue (interval_inside left (interval_intersect left (interval_hull left right))) =
  interval_inside_intersect_hull_left left right

theorem left_inside_hull_intersect_by_law
    : IsTrue (interval_inside left (interval_hull left (interval_intersect left right))) =
  interval_inside_hull_left left (interval_intersect left right)

theorem hull_intersect_inside_left_by_law
    : IsTrue (interval_inside (interval_hull left (interval_intersect left right)) left) =
  interval_hull_intersect_inside_left left right

theorem same_shape_hull_intersect_control
    : IsTrue (interval_inside (interval_hull left (interval_intersect left right)) left) =
  interval_hull_intersect_inside_left left right

theorem moved_shared_argument_value
    : Equal Bool (interval_inside (interval_hull (interval_intersect left right) right) left) False =
  Proved
```

### `stdlib/numerical/interval-intersect-hull-absorption-closed`

- spec: §8, `interval_intersect_inside_left` and
  `interval_inside_intersect_hull_left`.
- promise class: durable invariant.
- given: `left = [0,2]` and `right = [1,3]`.
- expect: the intersection of `left` with its hull with `right` is
  mutually included with `left`.
- measured: each direction is applied through its public law and its
  closed Boolean inclusion is independently computed with `Proved`.
- claimed: the first §8 absorption pair holds for these nondegenerate
  closed intervals.
- gap: these endpoints do not prove the generic theorems.
- why: `right` is not inside `left`, `left` is not inside `right`, and
  the endpoint results are independently checked.

### `stdlib/numerical/interval-hull-intersect-absorption-closed`

- spec: §8, `interval_inside_hull_left` and
  `interval_hull_intersect_inside_left`.
- promise class: durable invariant.
- given: the same `left` and `right` as the first case.
- expect: the hull of `left` with its intersection with `right` is
  mutually included with `left`.
- measured: each direction is applied through its public law and its
  closed Boolean inclusion is independently computed with `Proved`.
- claimed: the second §8 absorption pair holds for these nondegenerate
  closed intervals.
- gap: these endpoints do not prove the generic theorems.
- why: the intersection endpoints and the hull endpoints are checked
  independently, and neither operand interval includes the other.

## Isolated refused moved-argument claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. The accepted same-shape control uses the correct shared `left`
argument in the hull and applies `interval_hull_intersect_inside_left`.
The refused claim changes that shared argument to `right` in the
intersection. Its independently computed result is `[1,3]`, which is not
inside `left = [0,2]`. Require refusal at `seed_neg`, not at an import or
earlier declaration.

### `stdlib/numerical/reject-moved-shared-argument-absorption`

- spec: §8 specifies absorption with the shared interval first; it does
  not state absorption after moving that argument.
- promise class: durable invariant.
- given: the complete accepted client, including the same-shape control
  and the independent false value, followed by `seed_neg` alone.
- expect: reject the moved-shared-argument claim that
  `interval_hull (interval_intersect left right) right` is inside `left`.
- measured: the accepted same-shape law application succeeds, and
  `moved_shared_argument_value` independently computes `False`.
- claimed: the specified absorption does not license moving its shared
  interval argument.
- gap: this counterexample does not characterize every invalid
  rearrangement.
- why: the negative has the same proposition and `Proved` body shape as
  the accepted control; the changed interval expression yields the
  independently false endpoint comparison.

```ken
theorem seed_neg : IsTrue (interval_inside (interval_hull (interval_intersect left right) right) left) = Proved
```

## Evidence boundary

Architect's isolated D0 on base `fd2f575576aa2f5893054dcaf89e38243b739edc`
checked the generic §8 laws and a public client, with no new `Axiom` and
equal scratch trusted GlobalId sets (114 → 114). Its closed client used the
same `[0,2]` and `[1,3]` intervals, checked all four public directions, and
refused the moved-shared-argument claim after an accepted control. This D0
is Architect-reported scratch evidence; it does not establish execution of
this exact seed, the future catalog implementation or package test, official
conformance-runner execution, CI, publication, landing or landed trust.
