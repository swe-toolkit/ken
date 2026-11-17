# Stored idempotence of interval hull and intersection

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §19.

Every accepted row instantiates both premise-free §19 laws at one closed
interval. These cases exercise stored structural equality, not equality
of member sets or a normalization of rational spellings. They are closed
instances; they do not establish the generic laws.

`MkRational n k` represents `n/(Suc k)`. The intervals are
`half_two = [1/2, 2]`, `quarters_two = [2/4, 2]`, and the inverted
`inverted_two_half = [2, 1/2]`. The first two have equal-valued but
differently stored lower endpoints. They are mutually inside by endpoint
value, while their interval terms are not structurally equal. The mixed
hull retains the first interval's `1/2` spelling. The isolated refusal
uses the same equality form as the accepted `quarters_two` hull-law
application, but combines `half_two` with `quarters_two` and claims the
result is `quarters_two`.

## Accepted closed client

Accept this entire client before testing the isolated refusal. Append only
`seed_neg` from the refused block to a fresh complete copy.

```ken
import Data.Numeric.Rational (Rational, MkRational)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_hull,
    interval_intersect,
    interval_inside,
    interval_hull_idempotent,
    interval_intersect_idempotent)

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const half_two : Interval = MkInterval half two

const quarters_two : Interval = MkInterval two_quarters two

const inverted_two_half : Interval = MkInterval two half

theorem half_two_hull_idempotent
    : Equal Interval (interval_hull half_two half_two) half_two =
  interval_hull_idempotent half_two

theorem half_two_intersect_idempotent
    : Equal Interval (interval_intersect half_two half_two) half_two =
  interval_intersect_idempotent half_two

theorem quarters_two_hull_idempotent
    : Equal Interval (interval_hull quarters_two quarters_two) quarters_two =
  interval_hull_idempotent quarters_two

theorem quarters_two_intersect_idempotent
    : Equal Interval (interval_intersect quarters_two quarters_two) quarters_two =
  interval_intersect_idempotent quarters_two

theorem inverted_hull_idempotent
    : Equal Interval
        (interval_hull inverted_two_half inverted_two_half)
        inverted_two_half =
  interval_hull_idempotent inverted_two_half

theorem inverted_intersect_idempotent
    : Equal Interval
        (interval_intersect inverted_two_half inverted_two_half)
        inverted_two_half =
  interval_intersect_idempotent inverted_two_half

theorem half_two_inside_quarters_two
    : Equal Bool (interval_inside half_two quarters_two) True = Proved

theorem quarters_two_inside_half_two
    : Equal Bool (interval_inside quarters_two half_two) True = Proved

theorem mixed_hull_keeps_first_spelling
    : Equal Interval (interval_hull half_two quarters_two) half_two = Refl
```

### `stdlib/numerical/interval-hull-intersect-idempotent-closed`

- spec: §19, `interval_hull_idempotent` and
  `interval_intersect_idempotent`.
- promise class: durable invariant.
- given: the same-stored intervals `half_two`, `quarters_two`, and
  `inverted_two_half`.
- expect: each of the three intervals accepts applications of both
  public idempotence laws.
- measured: six closed law applications; the first two interval terms
  have mutual `True` endpoint inclusion; the mixed hull has the first
  operand's stored spelling.
- claimed: hull and intersection preserve the stored interval when
  combined with that same interval, including non-normal and inverted
  bounds.
- gap: these closed applications do not establish the generic
  quantifiers or equality of arbitrary equal-valued intervals.
- why: the same-operand rows exercise the §19 law applications, while
  the mutually-inside pair and mixed-hull output distinguish stored
  equality from value equivalence.

## Isolated refused mixed-spelling claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. `quarters_two_hull_idempotent` is the same-form positive
control: its proof term applies when both hull operands are that same
stored interval. The direct accepted hull result and mutual-inside
assertions establish that the refusal is not caused by unavailable
operations or unequal endpoint values.

### `stdlib/numerical/reject-hull-equal-value-spelling`

- spec: §19 and §2.1; `Equal Interval` observes stored endpoint
  spellings, while the hull selects the first lower spelling for these
  equal-valued operands.
- promise class: durable invariant.
- given: `half_two = [1/2, 2]` and `quarters_two = [2/4, 2]`.
- expect: reject the claim that their mixed hull is structurally equal
  to `quarters_two`.
- measured: the accepted prefix applies the `quarters_two` self-hull
  law, proves mutual endpoint inclusion, and checks that the mixed hull
  is stored as `half_two`.
- claimed: equal rational values do not make differently stored
  interval endpoints structurally equal.
- gap: this single pair is not a general decision procedure for
  structural equality or rational value equivalence.
- why: the negative has the same equality form as the accepted
  `quarters_two` self-hull application; only the mixed first operand
  changes, and the expected stored lower endpoint differs.

```ken
theorem seed_neg
    : Equal Interval (interval_hull half_two quarters_two) quarters_two =
  interval_hull_idempotent quarters_two
```

## Evidence boundary

The Architect's AC-0 scratch report on base `3477e1cd1` is separate
from execution of this seed. This file states authored expectations only;
no seed execution, package test, official conformance run, CI,
publication, landing, or landed trust measurement is claimed.
