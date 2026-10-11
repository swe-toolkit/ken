# Empirical distributions from non-empty samples

A non-empty sample can be read as an exact distribution: each observation keeps
its place and contributes one unit of natural weight. Counts and sums are
exact numerators; the total is a checked positive natural number.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Repeated observations are distinct occurrences. This package reads the ordered
`NonEmpty` sample, maps each element to a unit-weight entry and constructs an
`ExactDistribution` whose proof of positive total is checked by the existing
constructor. The Boolean event and natural score observations traverse the same
sample; they require no enumeration or equality decision on its element type.

## 2. Definition

The five public operations give the size, event count, score sum, unit weights
and distribution in that order. The private `indicator` and `count_list`
accumulate one for a matching event, whereas `sum_list` accumulates each score.
`unit_weights` reuses ordinary-list `map`. The private `empirical_tail`
constructs the positive distribution from the head and possibly empty tail;
prepending one weight carries the old total as the new predecessor witness.

```ken
import Algorithm.Statistics.Exact
  (ExactDistribution, MkExactDistribution, entries, total, event_mass, expectation_weight)

import Data.Collections.Derived (list_append, map, filter)

import Data.Collections.List (length)

import Data.Collections.NonEmpty
  (NonEmpty,
    nonempty_head,
    nonempty_tail,
    nonempty_to_list,
    nonempty_to_list_shape,
    nonempty_append)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Core.Logic.Transport (cong, trans, sym)

import Core.Classes.LawfulClasses (bool_cases)

pub fn sample_size (q : Type) (sample : NonEmpty q) : Nat = length q (nonempty_to_list q sample)

fn indicator (b : Bool) : Nat =
  match b {
    True ↦ Suc Zero;
    False ↦ Zero
  }

fn count_list (q : Type) (xs : List q) (event : q → Bool) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons x rest ↦ add (indicator (event x)) (count_list q rest event)
  }

theorem add_one_left (n : Nat) : Equal Nat (add (Suc Zero) n) (Suc n) =
  trans
    Nat
    (add (Suc Zero) n)
    (Suc (add Zero n))
    (Suc n)
    ((proof suc_l for add) Zero n)
    (cong Nat Nat (add Zero n) n Suc ((proof zero_l for add) n))

pub fn event_count (q : Type) (sample : NonEmpty q) (event : q → Bool) : Nat =
  count_list q (nonempty_to_list q sample) event

fn sum_list (q : Type) (xs : List q) (score : q → Nat) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons x rest ↦ add (score x) (sum_list q rest score)
  }

pub fn score_sum (q : Type) (sample : NonEmpty q) (score : q → Nat) : Nat =
  sum_list q (nonempty_to_list q sample) score

fn weighted_one (q : Type) (x : q) : Pair q Nat = mk_pair q Nat x (Suc Zero)

pub fn unit_weights (q : Type) (xs : List q) : List (Pair q Nat) =
  map q (Pair q Nat) (weighted_one q) xs

fn prepend_one (q : Type) (x : q) (d : ExactDistribution q) : ExactDistribution q =
  MkExactDistribution
    q
    (Cons (Pair q Nat) (weighted_one q x) (entries q d))
    (total q d)
    (add_one_left (total q d))

fn empirical_tail (q : Type) (x : q) (rest : List q) : ExactDistribution q =
  match rest {
    Nil ↦
      MkExactDistribution
        q
        (Cons (Pair q Nat) (weighted_one q x) (Nil (Pair q Nat)))
        Zero
        Proved;
    Cons y ys ↦ prepend_one q x (empirical_tail q y ys)
  }

pub fn empirical_distribution (q : Type) (sample : NonEmpty q) : ExactDistribution q =
  empirical_tail q (nonempty_head q sample) (nonempty_tail q sample)
```

## 3. Using it

Smart constructors supply a sample without exposing its constructor. Its
`empirical_distribution` is an ordinary exact distribution and can be passed
to the same event and score operations as any other positive distribution.

```ken example
import Data.Collections.NonEmpty (nonempty_singleton)

const observation : NonEmpty Bool = nonempty_singleton Bool True

const observation_size : Nat = sample_size Bool observation

const weighted_observation : ExactDistribution Bool = empirical_distribution Bool observation
```

## 4. Laws & proofs

`empirical_entries` uses induction on the tail, then the public shape theorem
to replace the structural head-and-tail list by the abstract sample's view.
The mapped unit weights retain every occurrence, including repeats.

```ken
theorem empirical_tail_entries_nil
      (q : Type) (x : q)
    : Equal
        (List (Pair q Nat))
        (entries q (empirical_tail q x (Nil q)))
        (unit_weights q (Cons q x (Nil q))) =
  Refl

theorem empirical_tail_entries
      (q : Type) (x : q) (rest : List q)
    : Equal
        (List (Pair q Nat))
        (entries q (empirical_tail q x rest))
        (unit_weights q (Cons q x rest)) =
  match rest {
    Nil ↦ empirical_tail_entries_nil q x;
    Cons y ys ↦
      cong
        (List (Pair q Nat))
        (List (Pair q Nat))
        (entries q (empirical_tail q y ys))
        (unit_weights q (Cons q y ys))
        (λmore. Cons (Pair q Nat) (weighted_one q x) more)
        (empirical_tail_entries q y ys)
  }

pub theorem empirical_entries
      (q : Type) (sample : NonEmpty q)
    : Equal
        (List (Pair q Nat))
        (entries q (empirical_distribution q sample))
        (unit_weights q (nonempty_to_list q sample)) =
  let
    h = nonempty_head q sample;
    rest = nonempty_tail q sample
  in
    trans
      (List (Pair q Nat))
      (entries q (empirical_distribution q sample))
      (unit_weights q (Cons q h rest))
      (unit_weights q (nonempty_to_list q sample))
      (empirical_tail_entries q h rest)
      (sym
        (List (Pair q Nat))
        (unit_weights q (nonempty_to_list q sample))
        (unit_weights q (Cons q h rest))
        (cong
          (List q)
          (List (Pair q Nat))
          (nonempty_to_list q sample)
          (Cons q h rest)
          (unit_weights q)
          (nonempty_to_list_shape q sample)))
```

The distribution's checked total is one more than the tail length. Applying
the same shape theorem to `length` identifies this total with `sample_size`.

```ken
theorem empirical_tail_size
      (q : Type) (x : q) (rest : List q)
    : Equal Nat (total q (empirical_tail q x rest)) (Suc (length q rest)) =
  match rest {
    Nil ↦ Proved;
    Cons y ys ↦
      let tail_total =
        total q (empirical_tail q y ys)
      in
        trans
          Nat
          (total q (empirical_tail q x (Cons q y ys)))
          (Suc tail_total)
          (Suc (Suc (length q ys)))
          (add_one_left tail_total)
          (cong Nat Nat tail_total (Suc (length q ys)) Suc (empirical_tail_size q y ys))
  }

pub theorem empirical_total_size
      (q : Type) (sample : NonEmpty q)
    : Equal Nat (total q (empirical_distribution q sample)) (sample_size q sample) =
  let
    h = nonempty_head q sample;
    rest = nonempty_tail q sample
  in
    trans
      Nat
      (total q (empirical_distribution q sample))
      (Suc (length q rest))
      (sample_size q sample)
      (empirical_tail_size q h rest)
      (sym
        Nat
        (length q (nonempty_to_list q sample))
        (length q (Cons q h rest))
        (cong
          (List q)
          Nat
          (nonempty_to_list q sample)
          (Cons q h rest)
          (length q)
          (nonempty_to_list_shape q sample)))
```

The event fold also agrees with `length` after the ordinary-list `filter`.
A Boolean case proof relates the indicator's addition to either a retained
head or a discarded one; induction then covers the whole sample. The mass
bridge itself follows the exact distribution's entry fold, transporting the
abstract sample list through the public shape theorem.

```ken
fn filter_front (q : Type) (b : Bool) (x : q) (tail : List q) : List q =
  match b {
    True ↦ Cons q x tail;
    False ↦ tail
  }

theorem filter_length_step
      (q : Type) (x : q) (b : Bool) (tail : List q)
    : Equal Nat (add (indicator b) (length q tail)) (length q (filter_front q b x tail)) =
  bool_cases
    b
    (λv. Equal Nat (add (indicator v) (length q tail)) (length q (filter_front q v x tail)))
    (add_one_left (length q tail))
    ((proof zero_l for add) (length q tail))

theorem count_list_filtered_length
      (q : Type) (xs : List q) (event : q → Bool)
    : Equal Nat (count_list q xs event) (length q (filter q event xs)) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      let filtered_tail =
        filter q event rest
      in
        trans
          Nat
          (count_list q (Cons q x rest) event)
          (add (indicator (event x)) (length q filtered_tail))
          (length q (filter q event (Cons q x rest)))
          (cong
            Nat
            Nat
            (count_list q rest event)
            (length q filtered_tail)
            (λn. add (indicator (event x)) n)
            (count_list_filtered_length q rest event))
          (filter_length_step q x (event x) filtered_tail)
  }

theorem count_step
      (q : Type) (x : q) (d : ExactDistribution q) (p : q → Bool)
    : Equal Nat
        (event_mass q (prepend_one q x d) p)
        (add (indicator (p x)) (event_mass q d p)) =
  Refl

theorem empirical_tail_count
      (q : Type) (x : q) (rest : List q) (event : q → Bool)
    : Equal Nat
        (event_mass q (empirical_tail q x rest) event)
        (count_list q (Cons q x rest) event) =
  match rest {
    Nil ↦ Refl;
    Cons y ys ↦
      trans
        Nat
        (event_mass q (empirical_tail q x (Cons q y ys)) event)
        (add (indicator (event x)) (event_mass q (empirical_tail q y ys) event))
        (count_list q (Cons q x (Cons q y ys)) event)
        (count_step q x (empirical_tail q y ys) event)
        (cong
          Nat
          Nat
          (event_mass q (empirical_tail q y ys) event)
          (count_list q (Cons q y ys) event)
          (λz. add (indicator (event x)) z)
          (empirical_tail_count q y ys event))
  }

pub theorem empirical_event_count
      (q : Type) (sample : NonEmpty q) (event : q → Bool)
    : Equal Nat
        (event_mass q (empirical_distribution q sample) event)
        (event_count q sample event) =
  let
    h = nonempty_head q sample;
    rest = nonempty_tail q sample
  in
    trans
      Nat
      (event_mass q (empirical_distribution q sample) event)
      (count_list q (Cons q h rest) event)
      (event_count q sample event)
      (empirical_tail_count q h rest event)
      (sym
        Nat
        (count_list q (nonempty_to_list q sample) event)
        (count_list q (Cons q h rest) event)
        (cong
          (List q)
          Nat
          (nonempty_to_list q sample)
          (Cons q h rest)
          (λxs. count_list q xs event)
          (nonempty_to_list_shape q sample)))
```

A unit weight times a score is that score. Induction on the tail and
congruence under addition connect the exact expectation numerator to the
sample's score fold.

```ken
theorem one_times (n : Nat) : Equal Nat (mul (Suc Zero) n) n =
  match n {
    Zero ↦ Proved;
    Suc pred ↦
      trans
        Nat
        (mul (Suc Zero) (Suc pred))
        (Suc (mul (Suc Zero) pred))
        (Suc pred)
        Refl
        (cong Nat Nat (mul (Suc Zero) pred) pred Suc (one_times pred))
  }

theorem empirical_tail_sum
      (q : Type) (x : q) (rest : List q) (score : q → Nat)
    : Equal Nat
        (expectation_weight q (empirical_tail q x rest) score)
        (sum_list q (Cons q x rest) score) =
  match rest {
    Nil ↦
      trans
        Nat
        (expectation_weight q (empirical_tail q x (Nil q)) score)
        (add (score x) Zero)
        (sum_list q (Cons q x (Nil q)) score)
        (cong
          Nat
          Nat
          (mul (Suc Zero) (score x))
          (score x)
          (λz. add z Zero)
          (one_times (score x)))
        Refl;
    Cons y ys ↦
      let tail_score =
        expectation_weight q (empirical_tail q y ys) score
      in
        trans
          Nat
          (expectation_weight q (empirical_tail q x (Cons q y ys)) score)
          (add (score x) tail_score)
          (sum_list q (Cons q x (Cons q y ys)) score)
          (cong
            Nat
            Nat
            (mul (Suc Zero) (score x))
            (score x)
            (λz. add z tail_score)
            (one_times (score x)))
          (cong
            Nat
            Nat
            tail_score
            (sum_list q (Cons q y ys) score)
            (λz. add (score x) z)
            (empirical_tail_sum q y ys score))
  }

pub theorem empirical_score_sum
      (q : Type) (sample : NonEmpty q) (score : q → Nat)
    : Equal Nat
        (expectation_weight q (empirical_distribution q sample) score)
        (score_sum q sample score) =
  let
    h = nonempty_head q sample;
    rest = nonempty_tail q sample
  in
    trans
      Nat
      (expectation_weight q (empirical_distribution q sample) score)
      (sum_list q (Cons q h rest) score)
      (score_sum q sample score)
      (empirical_tail_sum q h rest score)
      (sym
        Nat
        (sum_list q (nonempty_to_list q sample) score)
        (sum_list q (Cons q h rest) score)
        (cong
          (List q)
          Nat
          (nonempty_to_list q sample)
          (Cons q h rest)
          (λxs. sum_list q xs score)
          (nonempty_to_list_shape q sample)))
```

Appending samples preserves their ordinary-list view. Transporting that
checked equality through each observation reduces the three append laws to
list append, natural addition and a structural induction on the left list.

```ken
theorem count_list_append
      (q : Type) (xs : List q) (ys : List q) (event : q → Bool)
    : Equal Nat
        (count_list q (list_append q xs ys) event)
        (add (count_list q xs event) (count_list q ys event)) =
  match xs {
    Nil ↦
      sym
        Nat
        (add Zero (count_list q ys event))
        (count_list q ys event)
        ((proof zero_l for add) (count_list q ys event));
    Cons x rest ↦
      let
        h = indicator (event x);
        left = count_list q rest event;
        right = count_list q ys event
      in
        trans
          Nat
          (count_list q (list_append q (Cons q x rest) ys) event)
          (add h (add left right))
          (add (count_list q (Cons q x rest) event) right)
          (cong
            Nat
            Nat
            (count_list q (list_append q rest ys) event)
            (add left right)
            (λz. add h z)
            (count_list_append q rest ys event))
          ((proof assoc for add) h left right)
  }

theorem sum_list_append
      (q : Type) (xs : List q) (ys : List q) (score : q → Nat)
    : Equal Nat
        (sum_list q (list_append q xs ys) score)
        (add (sum_list q xs score) (sum_list q ys score)) =
  match xs {
    Nil ↦
      sym
        Nat
        (add Zero (sum_list q ys score))
        (sum_list q ys score)
        ((proof zero_l for add) (sum_list q ys score));
    Cons x rest ↦
      let
        h = score x;
        left = sum_list q rest score;
        right = sum_list q ys score
      in
        trans
          Nat
          (sum_list q (list_append q (Cons q x rest) ys) score)
          (add h (add left right))
          (add (sum_list q (Cons q x rest) score) right)
          (cong
            Nat
            Nat
            (sum_list q (list_append q rest ys) score)
            (add left right)
            (λz. add h z)
            (sum_list_append q rest ys score))
          ((proof assoc for add) h left right)
  }

pub theorem sample_size_append
      (q : Type) (xs : NonEmpty q) (ys : NonEmpty q)
    : Equal Nat
        (sample_size q (nonempty_append q xs ys))
        (add (sample_size q xs) (sample_size q ys)) =
  trans
    Nat
    (sample_size q (nonempty_append q xs ys))
    (length q (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys)))
    (add (sample_size q xs) (sample_size q ys))
    (cong
      (List q)
      Nat
      (nonempty_to_list q (nonempty_append q xs ys))
      (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys))
      (length q)
      (nonempty_append::list_view q xs ys))
    (list_append::length q (nonempty_to_list q xs) (nonempty_to_list q ys))

pub theorem event_count_append
      (q : Type) (xs : NonEmpty q) (ys : NonEmpty q) (event : q → Bool)
    : Equal Nat
        (event_count q (nonempty_append q xs ys) event)
        (add (event_count q xs event) (event_count q ys event)) =
  trans
    Nat
    (event_count q (nonempty_append q xs ys) event)
    (count_list q (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys)) event)
    (add (event_count q xs event) (event_count q ys event))
    (cong
      (List q)
      Nat
      (nonempty_to_list q (nonempty_append q xs ys))
      (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys))
      (λlst. count_list q lst event)
      (nonempty_append::list_view q xs ys))
    (count_list_append q (nonempty_to_list q xs) (nonempty_to_list q ys) event)

pub theorem score_sum_append
      (q : Type) (xs : NonEmpty q) (ys : NonEmpty q) (score : q → Nat)
    : Equal Nat
        (score_sum q (nonempty_append q xs ys) score)
        (add (score_sum q xs score) (score_sum q ys score)) =
  trans
    Nat
    (score_sum q (nonempty_append q xs ys) score)
    (sum_list q (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys)) score)
    (add (score_sum q xs score) (score_sum q ys score))
    (cong
      (List q)
      Nat
      (nonempty_to_list q (nonempty_append q xs ys))
      (list_append q (nonempty_to_list q xs) (nonempty_to_list q ys))
      (λlst. sum_list q lst score)
      (nonempty_append::list_view q xs ys))
    (sum_list_append q (nonempty_to_list q xs) (nonempty_to_list q ys) score)
```

## 5. Design notes

The list of entries is the support: duplicates are not aggregated or looked
up through a keyed map. `unit_weights` uses `map` rather than a second list
recursion, and the event fold has a checked `filter`/`length` counterpart.
The constructor proof makes the total positive even for a singleton sample;
it does not assert a total independently of the stored weights.

The total is a denominator for interpreting event and score numerators, not
a returned probability or mean. This package offers no rational carrier,
division, normalization, variance, approximate value, or sampling operation.

## 6. References

- [Empirical measure](https://en.wikipedia.org/wiki/Empirical_measure) —
  Wikipedia; assigning equal mass to observed occurrences.
- [Probability mass function](https://en.wikipedia.org/wiki/Probability_mass_function)
  — Wikipedia; finite discrete masses and event sums.
- *A First Course in Probability*, 10th edition — Sheldon Ross; introductory
  context for event counts and expectations.

## 7. Trust & derivation

The behavioral contract is [statistics §2](../../../../spec/50-stdlib/63-statistics.md)
and the five-case [seed](../../../../conformance/stdlib/statistics/seed-empirical-sample.md).
The public surface is five functions and seven checked laws. Private helpers
construct the positive distribution, count or sum lists, and prove the tail,
filter and append inductions. None is separately exported.

| Reader task | Section |
|---|---|
| Obtain a positive distribution | [Definition](#2-definition), [Using it](#3-using-it) |
| Read event or score numerators | [Motivation](#1-motivation), [Design notes](#5-design-notes) |
| Review laws and trust | [Laws & proofs](#4-laws--proofs), [Trust & derivation](#7-trust--derivation) |

The derivation imports `Algorithm.Statistics.Exact` for the proof-carrying
carrier and its observations; `Data.Collections.NonEmpty` for the sample and
its shape/append transports; `Data.Collections.Derived` for `map`, `filter` and
`list_append`; `Data.Collections.List` for `length`; natural arithmetic for
`add` and `mul`; and transport/Boolean case proofs for induction bridges.
No `Axiom`, postulate, primitive, foreign binding or new `trusted_base()`
entry is introduced. Assumptions inherited from imported providers remain
visible to clients. The seed and package acceptance target validate the
closed behavior, generic proof imports and roots-loaded trust delta.
