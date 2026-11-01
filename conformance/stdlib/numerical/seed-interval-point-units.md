# Interval point units by endpoint-value inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §10.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-POINT-UNITS.md`
(as amended by Steward, `evt_2v2rzk4vq7713`).

Every case promises a durable invariant. `MkRational n k` denotes
`n/(Suc k)`. The accepted client uses `i=[1/2,1]` for all four
public laws and the same-form one-unit control. It also checks that
multiplication by the point zero yields a false inclusion on `i`,
and applies one public law to the inverted interval `[1,1/2]`.

The zero-unit refusal below is appended alone to the complete accepted
client. Its `IsTrue` claim has the same form and interval as the
accepted one-unit control; only the point value changes. The separate
Boolean control establishes that the zero-unit inclusion is false.
The laws concern endpoint-value inclusion, not structural
`Equal Interval`; a finite instance does not prove a generic law.

## Accepted client

Check this entire client before appending the sole refusal below.
`measured` describes the intended oracle, not a report that this seed
was executed.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_one)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_mul, interval_point,
   interval_inside, interval_add_zero_l_inside,
   interval_add_zero_l_rev_inside, interval_mul_one_l_inside,
   interval_mul_one_l_rev_inside)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const i : Interval = MkInterval half rational_one
const inverted : Interval = MkInterval rational_one half

theorem add_zero_left :
  IsTrue (interval_inside (interval_add (interval_point rational_zero) i) i) =
  interval_add_zero_l_inside i

theorem add_zero_left_rev :
  IsTrue (interval_inside i (interval_add (interval_point rational_zero) i)) =
  interval_add_zero_l_rev_inside i

theorem multiply_one_left :
  IsTrue (interval_inside (interval_mul (interval_point rational_one) i) i) =
  interval_mul_one_l_inside i

theorem multiply_one_left_rev :
  IsTrue (interval_inside i (interval_mul (interval_point rational_one) i)) =
  interval_mul_one_l_rev_inside i

theorem one_unit_claim_accepts :
  IsTrue (interval_inside (interval_mul (interval_point rational_one) i) i) =
  Proved

theorem zero_unit_value_is_false :
  Equal Bool
    (interval_inside (interval_mul (interval_point rational_zero) i) i)
    False =
  Proved

theorem inverted_add_zero_left :
  IsTrue
    (interval_inside
      (interval_add (interval_point rational_zero) inverted)
      inverted) =
  interval_add_zero_l_inside inverted
```

### stdlib/numerical/interval-point-unit-laws

- spec: §10; `interval_add_zero_l_inside`,
  `interval_add_zero_l_rev_inside`, `interval_mul_one_l_inside`, and
  `interval_mul_one_l_rev_inside`.
- promise class: durable invariant.
- given: `i=[1/2,1]` and the inverted interval `[1,1/2]`.
- expect: the accepted client applies all four public laws to `i`,
  accepts the one-unit `Proved` control, checks the zero-unit result
  as `False`, and applies the addition law to the inverted interval.
- measured: the closed applications use the exact public theorem
  interfaces on the listed inputs; they are intended oracle examples,
  not a claim that this seed was run.
- claimed: both endpoint-value inclusion directions hold for adding
  point zero and multiplying by point one, including on an inverted
  interval.
- gap: these closed instances do not establish the generic laws for
  every interval; the checked public theorem bodies carry those
  quantifiers.
- why: the inverted row reaches a public law on an empty interval,
  while the unit discriminator holds the interval and claim form fixed.

## Isolated refused zero-unit claim

On a fresh complete copy of the accepted client, append only the
following theorem. The accepted one-unit `IsTrue` control and the
`False` Boolean control above are part of the same client. The refusal
must be `KernelRejected TypeMismatch`, with its span inside this claim.

### stdlib/numerical/reject-zero-point-multiplication-unit

- spec: §10 and the Steward's AC-2 amendment,
  `evt_2v2rzk4vq7713`.
- promise class: durable invariant.
- given: the same `i=[1/2,1]` used by the accepted
  `one_unit_claim_accepts` control.
- expect: the false zero-point multiplication inclusion is refused
  as `KernelRejected TypeMismatch`, with the diagnostic span inside
  this theorem claim.
- measured: the candidate-local package test exercises the same
  `Proved` pair and pins the refusal kind and span; this seed itself
  remains unexecuted.
- claimed: zero is not the multiplicative inclusion unit for this
  interval, whereas one is.
- gap: this counterexample distinguishes the two closed inputs but
  does not prove the generic one-unit laws.
- why: the control and refusal differ only in the point value; the
  separate Boolean assertion confirms the refused proposition is
  false rather than malformed.

```ken
theorem seed_neg :
  IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) i) =
  Proved
```

## Evidence boundary

Architect's AC-0 measured the one-unit `Proved` control as accepted,
the zero-unit claim as `KernelRejected TypeMismatch` with a span
inside the claim, and the inverted addition-law application as
accepted. The Foundation package test exercises corresponding
candidate-local clients, but does not execute this seed. This seed is
authored and not Ken-checked or officially conformance-run. No
official conformance-runner execution, CI, publication, landing, or
landed trust measurement is claimed.
