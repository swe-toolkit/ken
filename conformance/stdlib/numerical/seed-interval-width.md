# Truncated Rational subtraction and interval width

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §15, AC-1 child
`ff78993ee071169a5e621692ba6d6c3918b1d714`.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-WIDTH.md` AC-2.

These source-shaped closed cases exercise §15's point and ordered-sum
width laws. Closed applications do not establish the laws' generic
quantifiers. The separate nonzero value row matters because a
constant-zero width satisfies both laws. The isolated refusals check
both ordered-endpoint premises and a wrong width value. This seed
records expected behavior from §15; no seed execution is claimed.

`MkRational n k` represents `n/(Suc k)`. The ordered sum below is
`[1/2,3/2] + [1,2]`; its component widths are both one, so its width
is two by `interval_width_add`. The direct value control uses `[1,3]`:
its width is two, not the upper endpoint three.

## Accepted client

The full accepted client must check before adding the isolated refusal.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_add, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_point, interval_add, interval_width,
   interval_width_point, interval_width_add)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const left : Interval = MkInterval half three_halves
const right : Interval = MkInterval one two
const inverted : Interval = MkInterval two one

theorem point_width_is_zero :
  rational_equiv (interval_width (interval_point half)) rational_zero =
  interval_width_point half

theorem ordered_sum_width_is_additive :
  rational_equiv
    (interval_width (interval_add left right))
    (rational_add (interval_width left) (interval_width right)) =
  interval_width_add half three_halves one two Proved Proved

theorem nonzero_width_value_control :
  rational_equiv (interval_width (MkInterval one three)) two =
  Proved
```

### stdlib/numerical/interval-width-point-zero

- spec: §15, `interval_width_point`.
- promise class: durable invariant.
- given: the point interval at `1/2`.
- expect: applying the public point law proves its width equivalent to
  `rational_zero`.
- measured: one closed application of `interval_width_point`.
- claimed: every point interval has width zero by value.
- gap: this instance does not establish the generic law.
- why: it checks the public law at a concrete Rational value.

### stdlib/numerical/interval-width-ordered-sum

- spec: §15, `interval_width_add`.
- promise class: durable invariant.
- given: ordered intervals `[1/2,3/2]` and `[1,2]`.
- expect: the width of their pointwise sum is equivalent to the sum
  of their widths.
- measured: two closed order premises and one application of the
  public sum law.
- claimed: width is additive on ordered interval sums.
- gap: this instance does not establish the generic law or its
  two-premise boundary.
- why: both order premises are supplied separately at the exact
  theorem application.

### stdlib/numerical/interval-width-nonzero-value-control

- spec: §15, definition of `interval_width`.
- promise class: durable invariant.
- given: `[1,3]` with a nonzero lower endpoint.
- expect: its width is equivalent to two.
- measured: the closed value of `interval_width` against the fixed
  Rational value two.
- claimed: width observes both endpoints and computes truncated
  upper-minus-lower, rather than always returning zero or `hi`.
- gap: this single value does not characterize all intervals.
- why: the lower endpoint is nonzero and the expected width differs
  from both zero and the upper endpoint.

## Isolated refused sum-law applications

On a fresh complete copy of the accepted client, append exactly one
negative declaration at a time. Each uses the same public sum law as
the ordered positive case, with one operand inverted. The valid
operand's premise stays positive while the inverted operand's premise
is refused.

### stdlib/numerical/reject-sum-width-inverted-first-operand

- spec: §15, `interval_width_add`.
- promise class: durable invariant.
- given: `inverted = [2,1]` and ordered `right = [1,2]`.
- expect: the sum-law application is refused because the first
  interval's `rational_leq 2 1` premise is false.
- measured: the same theorem application accepts on the ordered
  `left` and `right` controls; here only its first operand is inverted.
- claimed: the first ordered-endpoint premise is required.
- gap: this refusal alone does not check the second premise.
- controls: the accepted ordered sum has the same operation and law
  application form, with both premises true.
- why: `[2,1] + [1,2]` has width zero, while the component widths sum
  to one.

```ken
-- Reject: the first interval is inverted.
theorem reject_sum_width_inverted_first_operand :
  rational_equiv
    (interval_width (interval_add inverted right))
    (rational_add (interval_width inverted) (interval_width right)) =
  interval_width_add two one one two Proved Proved
```

### stdlib/numerical/reject-sum-width-inverted-second-operand

- spec: §15, `interval_width_add`.
- promise class: durable invariant.
- given: ordered `left = [1/2,3/2]` and `inverted = [2,1]`.
- expect: the sum-law application is refused because the second
  interval's `rational_leq 2 1` premise is false.
- measured: the same theorem application accepts on the ordered
  `left` and `right` controls; here only its second operand is inverted.
- claimed: the second ordered-endpoint premise is required.
- gap: this refusal alone does not check the first premise.
- controls: the accepted ordered sum has the same operation and law
  application form, with both premises true.
- why: `[1/2,3/2] + [2,1]` has width zero, while the component widths
  sum to one.

```ken
-- Reject: the second interval is inverted.
theorem reject_sum_width_inverted_second_operand :
  rational_equiv
    (interval_width (interval_add left inverted))
    (rational_add (interval_width left) (interval_width inverted)) =
  interval_width_add half three_halves two one Proved Proved
```

## Isolated refused wrong-width value

Append only the negative declaration below to a fresh complete copy of
the accepted client. The accepted control uses the same interval and
proposition form; only the claimed Rational value changes.

### stdlib/numerical/reject-interval-width-wrong-value

- spec: §15, definition of `interval_width`.
- promise class: durable invariant.
- given: `[1,3]` and the accepted same-form value control above.
- expect: claiming its width is three is refused.
- measured: the accepted control proves width equivalent to two; the
  isolated claim asserts equivalence to three.
- claimed: the upper endpoint alone is not the interval's width.
- gap: this one false value does not characterize all wrong width
  definitions.
- controls: same input, operation, proposition form, and proof
  terminal; only the expected value differs.
- why: §15 defines width as `rational_sub hi lo`, so `[1,3]` has
  width two.

```ken
-- Reject: [1,3] has width two, not three.
theorem reject_interval_width_wrong_value :
  rational_equiv (interval_width (MkInterval one three)) three =
  Proved
```

## Evidence boundary

The cited AC-1 child supplies the normative §15 contract; the seed
branch does not change `/spec`. AC-0's scratch results ground the
ordered and point clients, but do not establish execution of this seed.
CV has not run Ken tests or the official conformance runner for this
child. No implementation, package-test, CI, publication, or landing
result is claimed. No external or `local/refs` material was consulted.
