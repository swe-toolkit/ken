# Rational-valued exact statistics

Format: `../../README.md`.
Spec: `spec/50-stdlib/63-statistics.md` §3.

These five cases are **authored behavioral oracles**, not executed
conformance results. Source-shaped scratch Exact (SHA256
`a362da43f932ab2ba287d0282a80f6548af97fb1d503a11b85fb1c561d2ce6ce`),
Empirical (SHA256
`bc0b0308c5a18202d64d82469b832594e27b49b80c2281eac8b97de616ba2391`),
and a separately imported consumer (SHA256
`487a197b7d0663cb2ea43f4702f3916a988c840fb54e696fd020a52ca86749cd`)
checked rc=0 on base `29fe3a597` with an existing Oct 12 CLI.
The seed fixtures and seventeen closed assertions were separately
extracted and checked (SHA256
`b344c66f0238cf901e7ee1b893e751244827daa331aa0950dc190c67c282145b`).
The extended source-shaped modules checked the generic numerator,
denominator, forward cross-bridge, mass-bound, probability-order and
complement laws, without adding an `Axiom`. A false value equivalence,
false expectation bound and weakened generic bridge premise each
kernel-rejected; originals rechecked rc=0. None of this is delivery
of the new CAT declarations, execution of this seed against delivered
packages, a fresh CLI build, Cargo tests, CI or publication. Every
case has promise class **durable invariant**.

## Shared fixtures

The first weight is one on `True`, the second weight is two on
`False`: their total is three, not the number of entries. The event
selects `True`, while the score maps `True` to two and `False` to one.
For the empirical sample, two `True` observations and one `False`
produce score sum five and sample size three. An independently
constructed `2/6` spelling has the same value as `1/3`; `1/2` does
not. The expected numerals are fixed arithmetic oracles, not values
read back from the adapter being checked.

```ken
import Algorithm.Statistics.Exact
  (ExactDistribution, MkExactDistribution, probability, expectation,
   total, event_mass, expectation_weight, probability_numerator,
   probability_denominator, expectation_numerator,
   expectation_denominator, probability_cross_bridge,
   expectation_cross_bridge, probability_mass_bound,
   probability_leq_one, probability_complement_one)
import Algorithm.Statistics.Empirical
  (empirical_distribution, empirical_mean, sample_size, score_sum,
   empirical_mean_numerator,
   empirical_mean_denominator, empirical_mean_cross_bridge)
import Data.Numeric.Rational
  (Rational, MkRational, numerator, denominator, rational_equiv,
   rational_leq, rational_add, rational_one)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons)
import Core.Classes.EffectfulClasses (compose)
import Core.Classes.LawfulClasses (bool_not)

const atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const distribution : ExactDistribution Bool =
  MkExactDistribution Bool atoms (Suc (Suc Zero)) Proved
const sample : NonEmpty Bool =
  nonempty_cons Bool True
    (Cons Bool True (Cons Bool False (Nil Bool)))
const is_true : Bool → Bool = λx. x
const score : Bool → Nat =
  λx. match x { True ↦ Suc (Suc Zero); False ↦ Suc Zero }
const two_sixths : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc (Suc (Suc Zero)))))
const one_half : Rational = MkRational (Suc Zero) (Suc Zero)
const four_thirds : Rational =
  MkRational (Suc (Suc (Suc (Suc Zero)))) (Suc (Suc Zero))
const five_thirds : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) (Suc (Suc Zero))
```

Here `MkRational n k` represents `n/(Suc k)`; in particular, the
stored predecessor for denominator six is five. The `Proved` total
witness is checked, not assumed. Neither sample construction nor any
law below requires `Finite Bool`, `DecEq Bool`, a second rational
carrier, normalization, division, or `Float`.

## Event probability as a value

### stdlib/statistics/rational-probability-one-third-two-sixths

- spec: `spec/50-stdlib/63-statistics.md` §§1.1 and 3.1.
- promise class: durable invariant.
- given: `distribution`, `is_true`, and independently constructed
  `two_sixths`. Keep the `False` weight at two and `True` at one.
- expect: `event_mass Bool distribution is_true = 1` and
  `total Bool distribution = 3`. `probability_numerator` yields one;
  `probability_denominator` yields three. The closed cross equation
  `mul 1 6 = mul 2 3` supplies `probability_cross_bridge`, so
  `rational_equiv (probability Bool distribution is_true)
  two_sixths` is proved. The returned carrier term need not be
  structurally `Equal Rational` to `two_sixths`.
- measured: both projected parts and a value-equivalence theorem to
  a differently spelled fraction with a nontrivial cross product.
- claimed: the returned rational agrees exactly with §1's event
  numerator over its positive total, independently of spelling.
- the gap: one closed pair cannot prove the generic projection or
  cross-bridge laws; their checked CAT bodies are required.
- why: reading the two-entry list length as total gives `1/2`,
  while comparing pair spellings literally rejects `1/3 ~ 2/6`.

## A natural score may have expectation above one

### stdlib/statistics/rational-expectation-four-thirds

- spec: `spec/50-stdlib/63-statistics.md` §§1.1 and 3.1.
- promise class: durable invariant.
- given: the same distribution, with `score True = 2` and
  `score False = 1`. Both observations contribute to the sum.
- expect: `expectation_weight Bool distribution score = 4`,
  while `total Bool distribution = 3`. The result's numerator is
  four and its denominator three by the checked projection laws.
  `expectation_cross_bridge` proves `rational_equiv
  (expectation Bool distribution score) four_thirds` using the
  fixed cross equation `mul 4 3 = mul 4 3` over the §1 total.
  `rational_leq (expectation Bool distribution score)
  rational_one = False`; there is **no** general expectation ≤ one
  law for unbounded natural scores.
- measured: natural-weight score sum and positive denominator, with
  a result above one rather than a rounded or bounded probability.
- claimed: an expectation returns the exact §1 numerator over its
  positive total, without treating all scores as Boolean events.
- the gap: a closed `4/3` does not prove the generic bridge;
  unrestricted-score bounds are deliberately not specified.
- why: an event-only count returns `1/3`, and an accidental
  probability clamp maps this `4/3` to a value at most one.

## Mean from a non-empty sample

### stdlib/statistics/rational-empirical-mean-five-thirds

- spec: `spec/50-stdlib/63-statistics.md` §§2.1–2.2 and 3.2.
- promise class: durable invariant.
- given: `sample` with two `True` and one `False`, under the same
  natural score. The sample's length is three.
- expect: `score_sum Bool sample score = 5` and
  `sample_size Bool sample = 3`. The result's checked numerator
  is five and its denominator three. `empirical_mean_cross_bridge`
  proves `rational_equiv (empirical_mean Bool sample score)
  five_thirds` from `mul 5 3 = mul 5 3`. The same value is
  `expectation Bool (empirical_distribution Bool sample) score`;
  there is no second independent mean constructor.
- measured: duplicate observations contribute two scores; the
  `NonEmpty` sample supplies a positive denominator indirectly
  through its exact distribution.
- claimed: the returned mean agrees with §2's score sum divided
  by its sample size, using §1's existing expectation construction.
- the gap: the closed sample does not prove the generic empirical
  score/size bridges or the adapter's three checked laws.
- why: deduplicating `True` gives the wrong size and numerator;
  returning a natural score sum alone loses the denominator.

## Inequivalent values must remain inequivalent

### stdlib/statistics/rational-one-third-not-one-half

- spec: `spec/50-stdlib/63-statistics.md` §§3.1 and 3.4.
- promise class: durable invariant.
- given: the same `probability Bool distribution is_true = 1/3`
  and independently constructed `one_half = 1/2`. They share
  numerator one but have different positive denominators.
- expect: `rational_equiv (probability Bool distribution is_true)
  one_half → Bottom` has a checked closed proof; the cross products
  are `mul 1 2 = 2` versus `mul 1 3 = 3`. A fabricated proof
  `= Proved` must kernel-reject, with the positive `1/3 ~ 2/6`
  case remaining accepted on the same returned value.
- measured: opposite equivalence results for a common-numerator
  pair and a scaled pair of differently spelled equivalent values.
- claimed: value equality is exact cross multiplication, not raw
  numerator equality, approximation, or carrier identity.
- the gap: the negative fixture establishes only this pair, not
  generic transitivity or every consumer's respect for equivalence.
- why: a numerator-only comparator wrongly accepts `1/3 ~ 1/2`;
  a structural comparator wrongly rejects `1/3 ~ 2/6`.

## Bounds and complements

### stdlib/statistics/rational-probability-bounded-complement

- spec: `spec/50-stdlib/63-statistics.md` §3.3.
- promise class: durable invariant.
- given: the same weighted distribution and event. The complement
  selects the `False` atom, with natural mass two and total three.
- expect: `probability_mass_bound Bool distribution is_true`
  checks `1 ≤ 3`; `probability_leq_one Bool distribution is_true`
  checks `rational_leq (probability Bool distribution is_true)
  rational_one = True`. `probability_complement_one` proves
  `rational_equiv (rational_add
  (probability Bool distribution is_true)
  (probability Bool distribution
    (compose Bool Bool Bool bool_not is_true))) rational_one`.
  The two returned fractions are `1/3` and `2/3`; their sum is
  one **by value**, not necessarily by the spelling `1/1`.
- measured: a nonzero event, a nonzero complement and their
  positive shared total, the Boolean order and exact sum-to-one.
- claimed: a probability is bounded by one and complements
  partition even when rational addition leaves a reducible pair.
- the gap: a closed complement partition does not establish the
  generic mass-bound, rational-order or complement proofs.
- why: dropping the complement yields `1/3`, and counting only
  list entries makes `1/2 + 1/2` look right on the wrong total.

## Coverage boundary

The three value-producing operations preserve the §1–2 natural
numerators and their checked positive denominators. The five cases
compare an equivalent pair with different spellings, an inequivalent
pair sharing a numerator, a non-probability expectation above one, a
non-empty repeated sample, and a nontrivial complement. Their
fixed-fixture arithmetic does not discharge the twelve generic
checked laws required by §3. Every oracle is about returned values,
relations or proof outcomes, not repository text, a package-delivery
milestone or a floating approximation.
