# Exact finite-support distributions

A distribution stores natural weights beside atoms and checks that their sum
is positive. Events and scores then compute exact natural numerators against
the same positive total; no floating-point values or rational carrier are
introduced.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A finite list of atoms can represent a distribution without enumerating its
carrier. Each entry contributes independently, so repeated atoms add and
zero-weight entries remain valid. The constructor requires a proof that the
sum of the list's weights is `Suc n`; an empty or all-zero list cannot supply
one. An event is a Boolean predicate, and a score produces a natural number.
Both observations sum over the original list, with the checked positive
`total` serving as their denominator when interpreting a probability or an
expectation.

## 2. Definition

The public declarations follow the exact-distribution contract: the carrier
and observations, the five named event and score builders, then nine laws.
Private list inductions and natural-arithmetic rearrangements connect those
laws to the carrier's stored entries. Every proof is a checked term.

```ken
import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_r)

import Core.Classes.LawfulClasses (bool_not, bool_or, bool_and, bool_cases)

import Core.Logic.Transport (cong, trans, sym)

import Core.Classes.EffectfulClasses (compose)

fn total_weight (q : Type) (xs : List (Pair q Nat)) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons entry rest ↦ add (pair_snd q Nat entry) (total_weight q rest)
  }

pub data ExactDistribution (q : Type) : Type where {
  MkExactDistribution :
    (entries : List (Pair q Nat))
    → (one_less_total : Nat)
    → Equal Nat (total_weight q entries) (Suc one_less_total)
    → ExactDistribution q
}

export MkExactDistribution

pub fn entries (q : Type) (d : ExactDistribution q) : List (Pair q Nat) =
  match d {
    MkExactDistribution xs n positive ↦ xs
  }

pub fn one_less_total (q : Type) (d : ExactDistribution q) : Nat =
  match d {
    MkExactDistribution xs n positive ↦ n
  }

pub fn total (q : Type) (d : ExactDistribution q) : Nat = total_weight q (entries q d)

fn event_weight (q : Type) (xs : List (Pair q Nat)) (event : q → Bool) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons entry rest ↦
      add
        (match event (pair_fst q Nat entry) {
          True ↦ pair_snd q Nat entry;
          False ↦ Zero
        })
        (event_weight q rest event)
  }

fn expectation_list (q : Type) (xs : List (Pair q Nat)) (score : q → Nat) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons entry rest ↦
      add
        (mul (pair_snd q Nat entry) (score (pair_fst q Nat entry)))
        (expectation_list q rest score)
  }

pub fn event_mass (q : Type) (d : ExactDistribution q) (event : q → Bool) : Nat =
  event_weight q (entries q d) event

pub fn expectation_weight (q : Type) (d : ExactDistribution q) (score : q → Nat) : Nat =
  expectation_list q (entries q d) score

pub fn always_true (q : Type) (x : q) : Bool = True

pub fn always_false (q : Type) (x : q) : Bool = False

pub fn event_union (q : Type) (p : q → Bool) (r : q → Bool) (x : q) : Bool = bool_or (p x) (r x)

pub fn constant_one (q : Type) (x : q) : Nat = Suc Zero

pub fn score_add (q : Type) (f : q → Nat) (g : q → Nat) (x : q) : Nat = add (f x) (g x)

pub theorem total_positive
      (q : Type) (d : ExactDistribution q)
    : Equal Nat (total q d) (Suc (one_less_total q d)) =
  match d {
    MkExactDistribution xs n positive ↦ positive
  }

pub theorem impossible_event
      (q : Type) (d : ExactDistribution q)
    : Equal Nat (event_mass q d (always_false q)) Zero =
  event_mass_impossible q (entries q d)

pub theorem sure_event
      (q : Type) (d : ExactDistribution q)
    : Equal Nat (event_mass q d (always_true q)) (total q d) =
  event_mass_sure q (entries q d)

pub theorem complement_mass
      (q : Type) (d : ExactDistribution q) (p : q → Bool)
    : Equal Nat
        (add (event_mass q d p) (event_mass q d (compose q Bool Bool bool_not p)))
        (total q d) =
  complement_partition q (entries q d) p

pub theorem disjoint_mass
      (q : Type)
      (d : ExactDistribution q)
      (p : q → Bool)
      (r : q → Bool)
      (disjoint : (x : q) → Equal Bool (bool_and (p x) (r x)) False)
    : Equal Nat
        (event_mass q d (event_union q p r))
        (add (event_mass q d p) (event_mass q d r)) =
  disjoint_event_additivity q (entries q d) p r disjoint

pub theorem expectation_unit
      (q : Type) (d : ExactDistribution q)
    : Equal Nat (expectation_weight q d (constant_one q)) (total q d) =
  expectation_one q (entries q d)

pub theorem expectation_linearity
      (q : Type) (d : ExactDistribution q) (f : q → Nat) (g : q → Nat)
    : Equal Nat
        (expectation_weight q d (score_add q f g))
        (add (expectation_weight q d f) (expectation_weight q d g)) =
  expectation_additive q (entries q d) f g

pub theorem event_mass_ext
      (q : Type)
      (d : ExactDistribution q)
      (p : q → Bool)
      (r : q → Bool)
      (same : (x : q) → Equal Bool (p x) (r x))
    : Equal Nat (event_mass q d p) (event_mass q d r) =
  event_weight_ext_list q (entries q d) p r same

pub theorem expectation_weight_ext
      (q : Type)
      (d : ExactDistribution q)
      (f : q → Nat)
      (g : q → Nat)
      (same : (x : q) → Equal Nat (f x) (g x))
    : Equal Nat (expectation_weight q d f) (expectation_weight q d g) =
  expectation_list_ext q (entries q d) f g same
```

## 3. Using it

The following closed example makes the denominator explicit. The event for
`True` has mass one and the total is two; the proof field is discharged by
reduction rather than an assumption.

```ken example
import Algorithm.Statistics.Exact (ExactDistribution, MkExactDistribution, total, event_mass)

const two_atoms : List (Pair Bool Nat) =
  Cons
    (Pair Bool Nat)
    (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc Zero)) (Nil (Pair Bool Nat)))

const two_atom_distribution : ExactDistribution Bool =
  MkExactDistribution Bool two_atoms (Suc Zero) Proved

const accepts_true : Bool → Bool = λx. x

theorem two_atom_total : Equal Nat (total Bool two_atom_distribution) (Suc (Suc Zero)) = Proved

theorem two_atom_true_mass
    : Equal Nat (event_mass Bool two_atom_distribution accepts_true) (Suc Zero) =
  Proved
```

## 4. Laws & proofs

`total_positive` projects the constructor's checked proof. The impossible,
sure and unit laws follow list induction; a per-atom Boolean partition and
reassociation of `add` prove the complement law. Disjoint union is obtained
from an overlap decomposition: a pointwise proof that every overlap is false
makes its mass zero. `expectation_linearity` distributes multiplication over
the sum of two scores. Both extensionality laws transport the checked
pointwise premise under the head atom's contribution and combine it with the
induction hypothesis for the tail. None assumes function extensionality.

```ken
theorem event_mass_impossible
      (q : Type) (xs : List (Pair q Nat))
    : Equal Nat (event_weight q xs (always_false q)) Zero =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      trans
        Nat
        (event_weight q (Cons (Pair q Nat) entry rest) (always_false q))
        (event_weight q rest (always_false q))
        Zero
        ((proof zero_l for add) (event_weight q rest (always_false q)))
        (event_mass_impossible q rest)
  }

theorem event_mass_sure
      (q : Type) (xs : List (Pair q Nat))
    : Equal Nat (event_weight q xs (always_true q)) (total_weight q xs) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      cong
        Nat
        Nat
        (event_weight q rest (always_true q))
        (total_weight q rest)
        (λn. add (pair_snd q Nat entry) n)
        (event_mass_sure q rest)
  }

theorem mul_one_right (w : Nat) : Equal Nat (mul w (Suc Zero)) w = (proof zero_l for add) w

theorem expectation_one
      (q : Type) (xs : List (Pair q Nat))
    : Equal Nat (expectation_list q xs (constant_one q)) (total_weight q xs) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      trans
        Nat
        (expectation_list q (Cons (Pair q Nat) entry rest) (constant_one q))
        (add (pair_snd q Nat entry) (expectation_list q rest (constant_one q)))
        (total_weight q (Cons (Pair q Nat) entry rest))
        (cong
          Nat
          Nat
          (mul (pair_snd q Nat entry) (Suc Zero))
          (pair_snd q Nat entry)
          (λw. add w (expectation_list q rest (constant_one q)))
          (mul_one_right (pair_snd q Nat entry)))
        (cong
          Nat
          Nat
          (expectation_list q rest (constant_one q))
          (total_weight q rest)
          (λn. add (pair_snd q Nat entry) n)
          (expectation_one q rest))
  }

fn choose_weight (w : Nat) (b : Bool) : Nat =
  match b {
    True ↦ w;
    False ↦ Zero
  }

theorem point_partition
      (w : Nat) (b : Bool)
    : Equal Nat (add (choose_weight w b) (choose_weight w (bool_not b))) w =
  bool_cases
    b
    (λv. Equal Nat (add (choose_weight w v) (choose_weight w (bool_not v))) w)
    Refl
    ((proof zero_l for add) w)

theorem swap_middle
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (add (add a b) c) (add (add a c) b) =
  trans
    Nat
    (add (add a b) c)
    (add a (add b c))
    (add (add a c) b)
    (sym Nat (add a (add b c)) (add (add a b) c) ((proof assoc for add) a b c))
    (trans
      Nat
      (add a (add b c))
      (add a (add c b))
      (add (add a c) b)
      (cong Nat Nat (add b c) (add c b) (λz. add a z) ((proof comm for add) b c))
      ((proof assoc for add) a c b))

theorem shuffle_add
      (a : Nat) (b : Nat) (c : Nat) (d : Nat)
    : Equal Nat (add (add a b) (add c d)) (add (add a c) (add b d)) =
  trans
    Nat
    (add (add a b) (add c d))
    (add (add (add a b) c) d)
    (add (add a c) (add b d))
    ((proof assoc for add) (add a b) c d)
    (trans
      Nat
      (add (add (add a b) c) d)
      (add (add (add a c) b) d)
      (add (add a c) (add b d))
      (cong Nat Nat (add (add a b) c) (add (add a c) b) (λz. add z d) (swap_middle a b c))
      (sym
        Nat
        (add (add a c) (add b d))
        (add (add (add a c) b) d)
        ((proof assoc for add) (add a c) b d)))

theorem complement_partition
      (q : Type) (xs : List (Pair q Nat)) (event : q → Bool)
    : Equal Nat
        (add (event_weight q xs event) (event_weight q xs (compose q Bool Bool bool_not event)))
        (total_weight q xs) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        w = pair_snd q Nat entry;
        b = event (pair_fst q Nat entry);
        tail_left = event_weight q rest event;
        tail_right = event_weight q rest (compose q Bool Bool bool_not event)
      in
        trans
          Nat
          (add
            (add (choose_weight w b) tail_left)
            (add (choose_weight w (bool_not b)) tail_right))
          (add
            (add (choose_weight w b) (choose_weight w (bool_not b)))
            (add tail_left tail_right))
          (add w (total_weight q rest))
          (shuffle_add (choose_weight w b) tail_left (choose_weight w (bool_not b)) tail_right)
          (trans
            Nat
            (add
              (add (choose_weight w b) (choose_weight w (bool_not b)))
              (add tail_left tail_right))
            (add w (add tail_left tail_right))
            (add w (total_weight q rest))
            (cong
              Nat
              Nat
              (add (choose_weight w b) (choose_weight w (bool_not b)))
              w
              (λz. add z (add tail_left tail_right))
              (point_partition w b))
            (cong
              Nat
              Nat
              (add tail_left tail_right)
              (total_weight q rest)
              (λz. add w z)
              (complement_partition q rest event)))
  }

fn event_overlap (q : Type) (p : q → Bool) (r : q → Bool) (x : q) : Bool = bool_and (p x) (r x)

theorem point_union_overlap
      (w : Nat) (b : Bool) (c : Bool)
    : Equal Nat
        (add (choose_weight w (bool_or b c)) (choose_weight w (bool_and b c)))
        (add (choose_weight w b) (choose_weight w c)) =
  bool_cases
    b
    (λv.
      (c0 : Bool)
        → Equal Nat
        (add (choose_weight w (bool_or v c0)) (choose_weight w (bool_and v c0)))
        (add (choose_weight w v) (choose_weight w c0)))
    (λc0.
      bool_cases
        c0
        (λv.
          Equal
            Nat
            (add (choose_weight w (bool_or True v)) (choose_weight w (bool_and True v)))
            (add (choose_weight w True) (choose_weight w v)))
        Refl
        Refl)
    (λc0.
      bool_cases
        c0
        (λv.
          Equal
            Nat
            (add (choose_weight w (bool_or False v)) (choose_weight w (bool_and False v)))
            (add (choose_weight w False) (choose_weight w v)))
        ((proof comm for add) w Zero)
        Proved)
    c

theorem event_union_overlap
      (q : Type) (xs : List (Pair q Nat)) (p : q → Bool) (r : q → Bool)
    : Equal Nat
        (add (event_weight q xs (event_union q p r)) (event_weight q xs (event_overlap q p r)))
        (add (event_weight q xs p) (event_weight q xs r)) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        w = pair_snd q Nat entry;
        b = p (pair_fst q Nat entry);
        c = r (pair_fst q Nat entry);
        tail_union = event_weight q rest (event_union q p r);
        tail_overlap = event_weight q rest (event_overlap q p r);
        tail_p = event_weight q rest p;
        tail_r = event_weight q rest r
      in
        trans
          Nat
          (add
            (add (choose_weight w (bool_or b c)) tail_union)
            (add (choose_weight w (bool_and b c)) tail_overlap))
          (add
            (add (choose_weight w (bool_or b c)) (choose_weight w (bool_and b c)))
            (add tail_union tail_overlap))
          (add (add (choose_weight w b) tail_p) (add (choose_weight w c) tail_r))
          (shuffle_add
            (choose_weight w (bool_or b c))
            tail_union
            (choose_weight w (bool_and b c))
            tail_overlap)
          (trans
            Nat
            (add
              (add (choose_weight w (bool_or b c)) (choose_weight w (bool_and b c)))
              (add tail_union tail_overlap))
            (add (add (choose_weight w b) (choose_weight w c)) (add tail_p tail_r))
            (add (add (choose_weight w b) tail_p) (add (choose_weight w c) tail_r))
            (trans
              Nat
              (add
                (add (choose_weight w (bool_or b c)) (choose_weight w (bool_and b c)))
                (add tail_union tail_overlap))
              (add (add (choose_weight w b) (choose_weight w c)) (add tail_union tail_overlap))
              (add (add (choose_weight w b) (choose_weight w c)) (add tail_p tail_r))
              (cong
                Nat
                Nat
                (add (choose_weight w (bool_or b c)) (choose_weight w (bool_and b c)))
                (add (choose_weight w b) (choose_weight w c))
                (λz. add z (add tail_union tail_overlap))
                (point_union_overlap w b c))
              (cong
                Nat
                Nat
                (add tail_union tail_overlap)
                (add tail_p tail_r)
                (λz. add (add (choose_weight w b) (choose_weight w c)) z)
                (event_union_overlap q rest p r)))
            (sym
              Nat
              (add (add (choose_weight w b) tail_p) (add (choose_weight w c) tail_r))
              (add (add (choose_weight w b) (choose_weight w c)) (add tail_p tail_r))
              (shuffle_add (choose_weight w b) tail_p (choose_weight w c) tail_r)))
  }

theorem choose_excluded
      (w : Nat) (b : Bool) (h : Equal Bool b False)
    : Equal Nat (choose_weight w b) Zero =
  J (λv _. Equal Nat (choose_weight w v) Zero) Proved (sym Bool b False h)

theorem excluded_event_zero
      (q : Type)
      (xs : List (Pair q Nat))
      (p : q → Bool)
      (excluded : (x : q) → Equal Bool (p x) False)
    : Equal Nat (event_weight q xs p) Zero =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        w = pair_snd q Nat entry;
        b = p (pair_fst q Nat entry);
        tail_mass = event_weight q rest p
      in
        trans
          Nat
          (add (choose_weight w b) tail_mass)
          tail_mass
          Zero
          (trans
            Nat
            (add (choose_weight w b) tail_mass)
            (add Zero tail_mass)
            tail_mass
            (cong
              Nat
              Nat
              (choose_weight w b)
              Zero
              (λz. add z tail_mass)
              (choose_excluded w b (excluded (pair_fst q Nat entry))))
            ((proof zero_l for add) tail_mass))
          (excluded_event_zero q rest p excluded)
  }

theorem disjoint_event_additivity
      (q : Type)
      (xs : List (Pair q Nat))
      (p : q → Bool)
      (r : q → Bool)
      (disjoint : (x : q) → Equal Bool (bool_and (p x) (r x)) False)
    : Equal Nat
        (event_weight q xs (event_union q p r))
        (add (event_weight q xs p) (event_weight q xs r)) =
  let
    lhs = event_weight q xs (event_union q p r);
    overlap = event_weight q xs (event_overlap q p r);
    rhs = add (event_weight q xs p) (event_weight q xs r)
  in
    trans
      Nat
      lhs
      (add lhs overlap)
      rhs
      (sym
        Nat
        (add lhs overlap)
        lhs
        (trans
          Nat
          (add lhs overlap)
          (add lhs Zero)
          lhs
          (cong
            Nat
            Nat
            overlap
            Zero
            (λz. add lhs z)
            (excluded_event_zero q xs (event_overlap q p r) (λx. disjoint x)))
          Refl))
      (event_union_overlap q xs p r)

theorem expectation_additive
      (q : Type) (xs : List (Pair q Nat)) (f : q → Nat) (g : q → Nat)
    : Equal Nat
        (expectation_list q xs (score_add q f g))
        (add (expectation_list q xs f) (expectation_list q xs g)) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        w = pair_snd q Nat entry;
        fv = f (pair_fst q Nat entry);
        gv = g (pair_fst q Nat entry);
        tail_sum = expectation_list q rest (score_add q f g);
        tail_f = expectation_list q rest f;
        tail_g = expectation_list q rest g
      in
        trans
          Nat
          (add (mul w (add fv gv)) tail_sum)
          (add (add (mul w fv) (mul w gv)) (add tail_f tail_g))
          (add (add (mul w fv) tail_f) (add (mul w gv) tail_g))
          (trans
            Nat
            (add (mul w (add fv gv)) tail_sum)
            (add (add (mul w fv) (mul w gv)) tail_sum)
            (add (add (mul w fv) (mul w gv)) (add tail_f tail_g))
            (cong
              Nat
              Nat
              (mul w (add fv gv))
              (add (mul w fv) (mul w gv))
              (λz. add z tail_sum)
              (mul_add_distrib_r w fv gv))
            (cong
              Nat
              Nat
              tail_sum
              (add tail_f tail_g)
              (λz. add (add (mul w fv) (mul w gv)) z)
              (expectation_additive q rest f g)))
          (sym
            Nat
            (add (add (mul w fv) tail_f) (add (mul w gv) tail_g))
            (add (add (mul w fv) (mul w gv)) (add tail_f tail_g))
            (shuffle_add (mul w fv) tail_f (mul w gv) tail_g))
  }

theorem event_weight_ext_list
      (q : Type)
      (xs : List (Pair q Nat))
      (p : q → Bool)
      (r : q → Bool)
      (same : (x : q) → Equal Bool (p x) (r x))
    : Equal Nat (event_weight q xs p) (event_weight q xs r) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        x = pair_fst q Nat entry;
        w = pair_snd q Nat entry;
        pb = p x;
        rb = r x;
        left_tail = event_weight q rest p;
        right_tail = event_weight q rest r
      in
        trans
          Nat
          (add (choose_weight w pb) left_tail)
          (add (choose_weight w rb) left_tail)
          (add (choose_weight w rb) right_tail)
          (cong
            Nat
            Nat
            (choose_weight w pb)
            (choose_weight w rb)
            (λz. add z left_tail)
            (cong Bool Nat pb rb (choose_weight w) (same x)))
          (cong
            Nat
            Nat
            left_tail
            right_tail
            (λz. add (choose_weight w rb) z)
            (event_weight_ext_list q rest p r same))
  }

theorem expectation_list_ext
      (q : Type)
      (xs : List (Pair q Nat))
      (f : q → Nat)
      (g : q → Nat)
      (same : (x : q) → Equal Nat (f x) (g x))
    : Equal Nat (expectation_list q xs f) (expectation_list q xs g) =
  match xs {
    Nil ↦ Proved;
    Cons entry rest ↦
      let
        x = pair_fst q Nat entry;
        w = pair_snd q Nat entry;
        fx = f x;
        gx = g x;
        left_tail = expectation_list q rest f;
        right_tail = expectation_list q rest g
      in
        trans
          Nat
          (add (mul w fx) left_tail)
          (add (mul w gx) left_tail)
          (add (mul w gx) right_tail)
          (cong
            Nat
            Nat
            (mul w fx)
            (mul w gx)
            (λz. add z left_tail)
            (cong Nat Nat fx gx (λz. mul w z) (same x)))
          (cong
            Nat
            Nat
            left_tail
            right_tail
            (λz. add (mul w gx) z)
            (expectation_list_ext q rest f g same))
  }
```

## 5. Design notes

The support is the list, not a `Finite q` enumeration or a map keyed by
`DecEq q`. Each repeated atom contributes its weight on each appearance.
Zero-weight atoms do not change mass, whereas an all-zero list cannot provide
the constructor's positive-total proof. Event probabilities and expectations
are exact numerator/denominator interpretations; the package exports no ratio
type, division, approximation or `Float`. Equal fractions can be compared by
cross multiplication at the client boundary. Binding a distribution is not a
raw concatenation of inner lists when their totals differ; it and a proper
common-denominator construction are outside this package.

## 6. References

- [Probability mass function](https://en.wikipedia.org/wiki/Probability_mass_function)
  — Wikipedia; finite discrete mass and normalization.
- [Expected value](https://en.wikipedia.org/wiki/Expected_value) — Wikipedia;
  weighted expectation of a discrete random variable.
- *A First Course in Probability*, 10th edition — Sheldon Ross; introductory
  context for event probabilities and expected values.

## 7. Trust & derivation

The behavioral contract is [statistics §1](../../../../spec/50-stdlib/63-statistics.md)
and the eight-case [seed](../../../../conformance/stdlib/statistics/seed-exact-distribution.md).
The public surface is `ExactDistribution`, `MkExactDistribution`, five
observations, five event/score builders and nine proved laws. The finite-list
sums, Boolean partitions, arithmetic rearrangements and congruence lemmas
remain private to the package.

| Reader task | Section |
|---|---|
| Construct a distribution | [Definition](#2-definition), [Using it](#3-using-it) |
| Interpret an event or score | [Motivation](#1-motivation), [Design notes](#5-design-notes) |
| Review laws and trust | [Laws & proofs](#4-laws--proofs), [Trust & derivation](#7-trust--derivation) |

The derivation imports `Data.Numeric.Nat.Arithmetic` for `add`, `mul` and
its checked arithmetic laws; `Core.Classes.LawfulClasses` for Boolean
operations and cases; `Core.Classes.EffectfulClasses.compose` for the
complement; and `Core.Logic.Transport` for equality reasoning. It adds no
`Axiom`, postulate, primitive, foreign binding or `trusted_base()` entry;
inherited trust in any imported provider remains visible to its consumers.
The seed and package acceptance target exercise the eight closed behaviors,
public proof clients, exact root-loaded ownership and local trust delta.
