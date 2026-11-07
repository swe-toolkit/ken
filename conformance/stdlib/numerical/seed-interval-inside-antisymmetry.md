# Mutual inclusion fixes interval endpoint values

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §14.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-INSIDE-ANTISYMMETRY.md`.
Ruling: Architect AC-0 `evt_4w1k09rrv4f8z`.

Status: authored; not executed. This closed seed applies both public endpoint
laws to differently spelled rational values. The generic client belongs to
the Foundation package test, as ruled at AC-0; it is not claimed here.

## Accepted client

The intervals `[1/2, 2]` and `[2/4, 4/2]` have different stored endpoint
spellings but equal endpoint values. Both inclusions hold, so the two laws
establish `rational_equiv` separately for the lower and upper endpoint pairs.
The reflexivity control also makes the refusal's conclusion independently
inhabited.

```ken
import Data.Numeric.Rational (Rational, MkRational, rational_equiv)

import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside,
   interval_inside_antisym_lo, interval_inside_antisym_hi)

const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const four_halves : Rational =
  MkRational (Suc (Suc (Suc (Suc Zero)))) (Suc Zero)

const interval_a : Interval = MkInterval half two
const interval_b : Interval = MkInterval two_quarters four_halves
const zero_one : Interval = MkInterval zero one
const zero_two : Interval = MkInterval zero two

theorem interval_a_inside_b :
  IsTrue (interval_inside interval_a interval_b) = Proved

theorem interval_b_inside_a :
  IsTrue (interval_inside interval_b interval_a) = Proved

theorem zero_equiv_zero :
  rational_equiv zero zero =
  (proof refl for rational_equiv) zero

theorem lower_endpoint_equiv :
  rational_equiv half two_quarters =
  interval_inside_antisym_lo
    half two two_quarters four_halves Proved Proved

theorem upper_endpoint_equiv :
  rational_equiv two four_halves =
  interval_inside_antisym_hi
    half two two_quarters four_halves Proved Proved
```

### stdlib/numerical/interval-inside-antisymmetry-lower

- spec: `spec/50-stdlib/68-numerical.md` §14.
- promise class: durable invariant.
- given: `interval_a` and `interval_b`, with both inclusion premises
  accepted by the closed rational comparisons.
- expect: the lower law returns `rational_equiv half two_quarters`, not
  structural equality of the stored `Rational` values.
- claimed: mutual interval inclusion makes the lower endpoints value-
  equivalent for every four rational endpoints.
- gap: this fixed pair does not establish the generic law.

### stdlib/numerical/interval-inside-antisymmetry-upper

- spec: `spec/50-stdlib/68-numerical.md` §14.
- promise class: durable invariant.
- given: the same `interval_a` and `interval_b` and both accepted
  inclusion premises.
- expect: the upper law returns `rational_equiv two four_halves`, not
  structural equality of the stored `Rational` values.
- claimed: mutual interval inclusion makes the upper endpoints value-
  equivalent for every four rational endpoints.
- gap: this fixed pair does not establish the generic law.

## Isolated refusal

Append only the declarations below to a fresh copy of the entire accepted
client. The first inclusion is true and the reverse inclusion is false. The
conclusion `rational_equiv zero zero` is already proved independently above;
the rejection must therefore occur at the second `Proved`, for the missing
reverse-inclusion premise.

### stdlib/numerical/reject-interval-inside-antisymmetry-one-way

- spec: `spec/50-stdlib/68-numerical.md` §14.
- promise class: durable invariant.
- given: `[0, 1]` inside `[0, 2]`, with the same low-endpoint law form
  as the accepted client.
- expect: accept the forward inclusion and confirm the reverse direction
  computes to `False`; reject the appended application at its second
  `Proved`, not at the law name or conclusion.
- control: `lower_endpoint_equiv` accepts the same law application form
  when both interval inclusions hold for equal endpoint values.
- claimed: a single direction of interval inclusion does not provide the
  endpoint antisymmetry law's two premises.
- gap: one false reverse-inclusion instance does not characterize all
  failing premise pairs.

```ken
theorem zero_one_inside_zero_two :
  IsTrue (interval_inside zero_one zero_two) = Proved

theorem zero_two_not_inside_zero_one :
  Equal Bool (interval_inside zero_two zero_one) False = Proved

theorem reject_one_way_lower_endpoint :
  rational_equiv zero zero =
  interval_inside_antisym_lo zero one zero two Proved Proved
```

## Evidence boundary

Architect AC-0 reports that both public laws and the closed D0 client were
checked on scratch, and that the one-way refusal reached the reverse-inclusion
premise. The generic-client red was a duplicate explicit `IsTrue` import in
the D0 harness, not a law failure; AC-0 moved the generic client to the
Foundation package test. These are attributed scratch results, not execution
of this seed child or an assembled candidate. CV has not run `ken check`,
semantic tests, or the official conformance runner. No CI, publication, or
landing is claimed. No external references or `local/refs/` were consulted.
