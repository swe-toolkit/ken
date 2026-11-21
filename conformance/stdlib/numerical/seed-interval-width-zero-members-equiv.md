# A zero-width interval fixes the value of its members

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §20, Spec child
`62bcdefba30195b945990ecfc545824fb1e34748`.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV.md`
AC-2. Architect AC-0: `evt_425dr8af5zcmp`.

This seed gives a closed accepted application and an isolated refused
application of `interval_width_zero_equiv`. The accepted interval stores
`1/2` and `2/4` as its endpoints; they are different Rational terms but
are equivalent by value. The refused interval has positive width. Both
applications use the same members, containment proofs, claimed conclusion,
and theorem application form. They differ only in the interval argument.
The §64 constructor uses a denominator predecessor, so `half` and
`two_quarters` both denote `1/2`, while `one` denotes `1`.

Closed applications instantiate the public theorem; they do not establish
its generic quantifier. The Architect's AC-0 scratch results are separate
evidence and do not establish execution of this seed. No Ken check or
official conformance execution is claimed.

## Accepted client

Check this complete client before adding the isolated refusal.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_width,
   interval_width_zero_equiv)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const three_pred : Nat = Suc (Suc (Suc Zero))
const two_quarters : Rational = MkRational (Suc (Suc Zero)) three_pred
const one : Rational = MkRational (Suc Zero) Zero

const zero_width : Interval = MkInterval half two_quarters
const positive_width : Interval = MkInterval half one

theorem members_are_value_equivalent :
  rational_equiv half two_quarters = Proved

theorem zero_width_is_equivalent_to_zero :
  rational_equiv (interval_width zero_width) rational_zero = Proved

theorem zero_width_contains_half :
  IsTrue (interval_contains zero_width half) = Proved

theorem zero_width_contains_two_quarters :
  IsTrue (interval_contains zero_width two_quarters) = Proved

theorem positive_width_is_half :
  rational_equiv (interval_width positive_width) half = Proved

theorem positive_width_contains_half :
  IsTrue (interval_contains positive_width half) = Proved

theorem positive_width_contains_two_quarters :
  IsTrue (interval_contains positive_width two_quarters) = Proved

theorem accepted_zero_width_application :
  rational_equiv half two_quarters =
  interval_width_zero_equiv zero_width half two_quarters Proved Proved Proved
```

### stdlib/numerical/interval-width-zero-members-equivalent

- spec: §20, `interval_width_zero_equiv`.
- promise class: durable invariant.
- given: `[1/2,2/4]` and the contained members `1/2` and `2/4`.
- expect: the public theorem proves `rational_equiv half two_quarters`.
- measured: the closed zero-width premise, both containment premises, and
  one application of the public theorem; the conclusion also has an
  independent closed proof.
- claimed: a zero-width interval makes any two of its members equivalent
  by rational value.
- gap: this closed instance does not establish the generic theorem.
- why: the stored endpoint and member terms use different Rational
  representations while the public relation is `rational_equiv`, not
  structural equality.

## Isolated refused application

Append the following declaration alone to a fresh complete copy of the
accepted client. Do not include `accepted_zero_width_application` in the
negative copy. The accepted and refused theorem declarations have the
same form, conclusion, member arguments, and proof arguments; only the
interval argument changes.

```ken
-- Reject: positive-width interval supplied to the zero-width theorem.
theorem reject_positive_width_application :
  rational_equiv half two_quarters =
  interval_width_zero_equiv positive_width half two_quarters Proved Proved Proved
```

### stdlib/numerical/interval-width-positive-does-not-enable-zero-law

- spec: §20, `interval_width_zero_equiv`.
- promise class: durable invariant.
- given: `[1/2,1]` with the same members `1/2` and `2/4`.
- expect: the application is rejected at the first, zero-width premise.
- measured: the law is supplied `Proved` for that premise, followed by
  the same two `Proved` containment arguments and the same conclusion as
  the accepted application.
- claimed: the law cannot be applied when the interval has positive width.
- gap: one positive-width interval does not characterize all nonzero
  widths.
- controls: the accepted application has the same form with only the
  interval changed; the accepted prefix separately proves the positive
  interval has width equivalent to `1/2`, contains both members, and the
  members are equivalent.
- why: §15 defines width as truncated upper-minus-lower; `[1/2,1]` has
  positive width `1/2`. Thus the supplied `Proved` cannot establish the
  theorem's zero-width premise, while the containment and conclusion
  controls remain true.

## Evidence boundary

The Spec child supplies the normative §20 contract, and Architect AC-0
supplies a separately reported scratch development. This CV child changes
only `/conformance`. The seed has not been run as Ken or through the
official conformance runner. No catalog implementation, package test, CI,
publication, landing, or landed trust measurement is claimed.
