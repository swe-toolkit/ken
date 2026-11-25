# Rational interval split width

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §22.

These cases exercise the public theorem `interval_split_width`. For ordered
`lo ≤ m ≤ hi`, it relates the sum of the widths of `[lo,m]` and `[m,hi]`
to the width of `[lo,hi]` by rational value. The closed accepted client
uses `[1,5]` split at `2`, whose half-widths sum to `4`. Its false-total
control changes the claimed total to `3` and is not evidence for the theorem's
generic statement. The open client applies the public theorem to arbitrary
endpoints and split point. A separate refusal supplies a false upper-order
premise. These seed cases have not been executed as Ken or by an official
conformance runner.

## Shared accepted prefix

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_equiv, rational_leq,
   rational_of_nat)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_width, interval_split_width)

const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const five : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) Zero
const seven : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))) Zero

theorem split_width_four
    : rational_equiv
        (rational_add
          (interval_width (MkInterval one two))
          (interval_width (MkInterval two five)))
        (rational_of_nat (Suc (Suc (Suc (Suc Zero))))) =
  (proof trans for rational_equiv)
    (rational_add
      (interval_width (MkInterval one two))
      (interval_width (MkInterval two five)))
    (interval_width (MkInterval one five))
    (rational_of_nat (Suc (Suc (Suc (Suc Zero)))))
    (interval_split_width one five two Proved Proved)
    Proved

theorem split_width_open_swap (lo : Rational) (hi : Rational) (m : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → rational_equiv
        (interval_width (MkInterval lo hi))
        (rational_add
          (interval_width (MkInterval lo m))
          (interval_width (MkInterval m hi))) =
  λlower.
    λupper.
      (proof sym for rational_equiv)
        (rational_add
          (interval_width (MkInterval lo m))
          (interval_width (MkInterval m hi)))
        (interval_width (MkInterval lo hi))
        (interval_split_width lo hi m lower upper)
```

### stdlib/numerical/interval-split-width-four

- spec: `spec/50-stdlib/68-numerical.md` §22.
- promise class: durable invariant.
- given: the public theorem applied to `[1,5]` split at `2`.
- expect: `split_width_four` accepts, establishing that the sum of the two
  half-widths is equivalent to `rational_of_nat 4`.
- measured: the closed client uses `interval_split_width` to relate the sum
  to the whole interval's width, then proves that width equivalent to four.
- claimed: the public law supports this ordered split and total.
- gap: this closed instance does not establish the theorem for arbitrary
  endpoints and split points.
- why: the half-widths are `1` and `3`, and the whole width is `4`.

### stdlib/numerical/interval-split-width-open-client

- spec: `spec/50-stdlib/68-numerical.md` §22.
- promise class: durable invariant.
- given: arbitrary `lo`, `hi`, and `m`, with proofs of `lo ≤ m` and `m ≤ hi`.
- expect: `split_width_open_swap` accepts by applying the imported theorem and
  symmetry of `rational_equiv`.
- measured: the open client applies `interval_split_width lo hi m lower upper`
  at its exact public type.
- claimed: a client can use the public law with its two ordered premises.
- gap: this client uses the theorem; it does not independently prove its
  generic body.
- why: the theorem application is on free endpoints and split point, not just
  the closed `[1,5]` example.

## False-total control on a fresh accepted prefix

Append only the following declaration to a fresh copy of the accepted prefix
above. It has the same law application and proof route as `split_width_four`;
the claimed total is the only changed value.

```ken
theorem split_width_three_control
    : rational_equiv
        (rational_add
          (interval_width (MkInterval one two))
          (interval_width (MkInterval two five)))
        (rational_of_nat (Suc (Suc (Suc Zero)))) =
  (proof trans for rational_equiv)
    (rational_add
      (interval_width (MkInterval one two))
      (interval_width (MkInterval two five)))
    (interval_width (MkInterval one five))
    (rational_of_nat (Suc (Suc (Suc Zero))))
    (interval_split_width one five two Proved Proved)
    Proved
```

### stdlib/numerical/interval-split-width-false-total-control

- spec: `spec/50-stdlib/68-numerical.md` §22.
- promise class: durable invariant.
- control role: soundness control, not evidence for the theorem's generic
  statement.
- given: the same `[1,5]` split at `2`, same theorem application, and same
  proof route as the accepted total-four case.
- expect: the isolated declaration is refused because the resulting width
  cannot be equivalent to `rational_of_nat 3`.
- measured: the law application yields the whole width; the final closed
  claim asks for a different rational value.
- claimed: the accepted total-four proof cannot establish the false total-three
  claim.
- gap: this false-total control checks neither the theorem's generic
  quantifiers nor either ordered premise.
- controls: the total-four case accepts on the same prefix and proof route; the
  claimed total is the only changed value.
- why: `[1,2]` has width `1`, `[2,5]` has width `3`, and `[1,5]` has width
  `4`, not `3`. The refusal is at the false closed equality, not at the law
  application.

## Refusal of the upper-order premise

Append only the declaration below to a fresh copy of the accepted prefix. The
accepted total-four client uses the same endpoints and proof forms with split
point `2`; this attempt changes the split point to `7`, making `7 ≤ 5` false.

```ken
theorem split_width_m_above_hi_refused
    : rational_equiv
        (rational_add
          (interval_width (MkInterval one seven))
          (interval_width (MkInterval seven five)))
        (interval_width (MkInterval one five)) =
  interval_split_width one five seven Proved Proved
```

### stdlib/numerical/interval-split-width-premise-refusal

- spec: `spec/50-stdlib/68-numerical.md` §22.
- promise class: durable invariant.
- given: the law application with `lo = 1`, `hi = 5`, and `m = 7`.
- expect: the isolated declaration is refused when supplying `Proved` for the
  false premise `rational_leq seven five`; do not pin diagnostic wording or
  span.
- measured: the first order premise is true, while the second is false; the
  accepted split-at-two control supplies the same premise forms successfully.
- claimed: the law requires the upper-order premise `m ≤ hi`.
- gap: this premise refusal does not establish the theorem's generic body or
  characterize other false inputs.
- controls: `split_width_four` accepts with the same endpoints and both
  premises supplied by `Proved`; only the split point differs.
- why: truncation makes the upper interval `[7,5]` have width zero, so the
  widths `6` and `0` sum to `6`, not the whole width `4`. The false second
  premise is encountered at the theorem application.

The false-total row is a soundness control, not a theorem discriminator. The
open client tests use of the public generic statement; the closed control tests
a false result; the premise refusal isolates the `m ≤ hi` input boundary.
Architect AC-0 measured the proof forms on scratch, not in this seed. This file
has not been executed as Ken or by an official conformance runner; no CI,
publication, or landing is claimed.
