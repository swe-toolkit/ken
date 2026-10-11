# Exact statistics from non-empty samples

Format: `../../README.md`.
Spec: `spec/50-stdlib/63-statistics.md` §2.

These five cases are **authored behavioral oracles**, not executed
results. The first-party `Algorithm.Statistics.Empirical` scratch
(SHA256 `6fe07c50c902311058c99cf7f3ade615b850fed70f1302c4567a962c7b01a95b`)
checked rc=0 against the landed `Algorithm.Statistics.Exact` source and
`Data.Collections.NonEmpty.nonempty_to_list_shape` on main
`e49cd5b95`; it includes the seven generic law bodies and no `Axiom`.
A separate scratch consumer (SHA256
`adac6a14534cbee4dc4eef2ae4d3bcdf37271a9b68f40bb2538a111587faf185`)
checked twenty-one closed fixture claims and seven generic imported-law
clients rc=0. Three independent false-law/weight mutations rejected;
those checks are not conformance execution, a delivered empirical CAT
package, fresh CLI build, Cargo tests, CI, or publication. Every case
has promise class **durable invariant**. The cases assert values and
relations, not repository-text shape or an implementation milestone.

## Shared fixtures

Use the existing `NonEmpty Bool` smart constructors and append, without
exporting or matching `NonEmptyCons`. The `True` event selects exactly
`True`, and the score assigns `True` two and `False` one. The seed
reads the empirical package's public operations and §1's existing
`total`, `entries`, `event_mass`, and `expectation_weight`.

```ken
import Algorithm.Statistics.Empirical
  (sample_size, event_count, score_sum, unit_weights,
   empirical_distribution, empirical_entries, empirical_total_size,
   empirical_event_count, empirical_score_sum,
   sample_size_append, event_count_append, score_sum_append)
import Algorithm.Statistics.Exact
  (entries, total, event_mass, expectation_weight, constant_one)
import Data.Collections.NonEmpty
  (NonEmpty, nonempty_singleton, nonempty_cons, nonempty_append,
   nonempty_to_list)

const lone : NonEmpty Bool = nonempty_singleton Bool True
const duplicate : NonEmpty Bool =
  nonempty_cons Bool True (Cons Bool True (Nil Bool))
const misses : NonEmpty Bool =
  nonempty_cons Bool False (Cons Bool False (Nil Bool))
const joined : NonEmpty Bool = nonempty_append Bool duplicate misses
const is_true : Bool → Bool = λx. x
const score : Bool → Nat =
  λx. match x { True ↦ Suc (Suc Zero); False ↦ Suc Zero }
```

The `Proved` notation in the assertions below denotes a closed,
checker-dischargeable fact about these fixtures, not a general proof
substituting for one of the package's seven checked public theorems.
The fixtures preserve duplicates and their order. No `Finite Bool`,
`DecEq Bool`, rational, division, or `Float` argument is supplied.

## One observation

### stdlib/statistics/empirical-singleton-positive-total

- spec: `spec/50-stdlib/63-statistics.md` §§2.1–2.2.
- promise class: durable invariant.
- given: `lone`, one `True` observation.
- expect: `sample_size Bool lone = 1`, `total Bool
  (empirical_distribution Bool lone) = 1`, and `event_count Bool
  lone is_true = event_mass Bool (empirical_distribution Bool lone)
  is_true = 1`. `empirical_total_size` and `empirical_event_count`
  apply without a possible empty sample or an unchecked proof field.
- measured: the one-observation size, total and two matching event
  counts on a positive exact distribution.
- claimed: the structural head supplies one unit of mass, including
  when the ordinary list tail is empty.
- the gap: this closed head does not prove positivity or the two
  bridges for arbitrary `NonEmpty q`; CAT owes their checked bodies.
- why: an off-by-one interpretation of the empty tail returns zero
  for at least one of the three positive observations.

## Repeated observations

### stdlib/statistics/empirical-repeated-true-adds-twice

- spec: `spec/50-stdlib/63-statistics.md` §§2.1–2.2.
- promise class: durable invariant.
- given: `duplicate`, whose head and one tail entry are both `True`.
  Compare to `lone` under the same `is_true` event.
- expect: `sample_size Bool duplicate = 2` and `event_count Bool
  duplicate is_true = event_mass Bool (empirical_distribution Bool
  duplicate) is_true = 2`; the corresponding `lone` values are `1`.
  `empirical_entries Bool duplicate` checks that its stored list is
  `unit_weights Bool (nonempty_to_list Bool duplicate)`, two separate
  unit-weight entries, not one aggregated or deduplicated entry.
- measured: two occurrences of one value have count/mass `2`, above
  one, and a checked representation equality preserves both entries.
- claimed: occurrence multiplicity determines empirical mass, not
  distinct-vertex count or last-wins lookup.
- the gap: a two-item Bool fixture cannot prove the generic list
  representation; the package's `empirical_entries` theorem must.
- why: deduplication returns `1` for this event despite sample size
  `2`; a weight of two on one atom can match the mass but violates
  the separate-entry equality.

## Unsatisfied event

### stdlib/statistics/empirical-no-observation-matches

- spec: `spec/50-stdlib/63-statistics.md` §§2.1–2.2.
- promise class: durable invariant.
- given: `misses`, two `False` observations; keep `is_true` fixed
  from the repeated case. The input is nonempty and its total is `2`.
- expect: `event_count Bool misses is_true = 0` and `event_mass Bool
  (empirical_distribution Bool misses) is_true = 0`, while
  `sample_size Bool misses = total Bool (empirical_distribution Bool
  misses) = 2`.
- measured: zero event numerator alongside nonzero size and
  denominator for the same sample.
- claimed: an event counts exactly the satisfying observations;
  failing observations still contribute to positive total mass.
- the gap: a closed predicate cannot stand in for the package's
  checked arbitrary-event bridge theorem.
- why: counting every observation as a match yields `2` rather than
  `0`; dropping failures from total yields `0` instead of `2`.

## Appended samples

### stdlib/statistics/empirical-append-preserves-both-contributions

- spec: `spec/50-stdlib/63-statistics.md` §2.2.
- promise class: durable invariant.
- given: `joined = nonempty_append Bool duplicate misses`; left has
  two matching `True` observations and right has two nonmatching
  `False` observations. Check both sides independently before append.
- expect: `sample_size Bool joined = 4 = add 2 2` and
  `event_count Bool joined is_true = 2 = add 2 0`. The exact
  distribution has total `4` and `True` mass `2`.
  `sample_size_append Bool duplicate misses` and
  `event_count_append Bool duplicate misses is_true` prove the
  corresponding equalities for these fixtures.
- measured: append adds two nonempty sizes, and the selected event
  keeps the left's positive count while the right contributes zero.
- claimed: `nonempty_append` preserves the ordered occurrence list;
  size and event count add for arbitrary samples.
- the gap: a closed `2+2` and `2+0` cannot prove generic append
  additivity; the checked CAT laws consume the existing
  `nonempty_append::list_view` proof.
- why: dropping the right input yields size `2`, and counting every
  right-hand observation as `True` yields event count `4`.

## Natural-score sum

### stdlib/statistics/empirical-score-sums-unit-weight-entries

- spec: `spec/50-stdlib/63-statistics.md` §§2.1–2.2.
- promise class: durable invariant.
- given: the mixed `joined`, with two `True` observations scored `2`
  and two `False` observations scored `1`. Compare the same sample
  under `constant_one Bool`; retain its positive total `4`.
- expect: `score_sum Bool duplicate score = 4`, `score_sum Bool
  misses score = 2`, `score_sum Bool joined score = 6 = add 4 2`;
  `expectation_weight Bool (empirical_distribution Bool joined)
  score = 6` and `expectation_weight Bool (empirical_distribution
  Bool joined) (constant_one Bool) = 4`. `score_sum_append Bool
  duplicate misses score` and `empirical_score_sum Bool joined
  score` provide checked closed instances of the generic laws.
- measured: an unequal-score mixture's sum and §1 expectation
  numerator, each contrasted with the unit-score numerator and total.
- claimed: each occurrence contributes exactly its natural score
  at unit weight; append adds scores without dividing by sample size.
- the gap: closed numerators cannot prove the score bridge or
  append law for all scores and carriers; CAT must check both.
- why: counting observations alone gives `4`, not `6`; counting
  only the selected event gives `4`, not `6`; normalizing to a
  returned mean would no longer produce the natural numerator `6`.

## Coverage boundary

The five cases cover one observation and positive total, duplicates
with a count above one, an event no observation satisfies, an
asymmetric append, and a mixed natural-score sum. In each case the
independent expected values are fixed-fixture arithmetic; the §1
bridge equalities and seven generic §2 laws remain checked package
obligations. Approximate values, means as values, variance, sampling
from a distribution, rational values, `pure`/`bind`, and `Float`
remain outside this seed. No assertion depends on source text,
private constructors, or a catalog completion status.
