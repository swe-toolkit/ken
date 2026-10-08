# `Dfa` — deterministic automata over arbitrary state and input types

A deterministic automaton runs a finite word through a total transition and
reports whether the resulting state is final. Its state space need not be
finite. Complement changes the final predicate; intersection and union
combine two machines that consume the same input.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

`accepts_complement`, `accepts_intersection`, and `accepts_union` connect
three useful constructions to Boolean negation, conjunction, and
disjunction. Their general foundation is a runner from **any** state and a
product automaton whose state contains one state from each operand.

A finite word ensures the runner terminates, not that its state carrier is
finite. The transition is a function rather than a lookup table; no equality
decision on either the state or the alphabet is required.

## 2. Definition

`Dfa q a` stores a transition, an initial state, and a Boolean final-state
test. `run` processes the word from the supplied state, whereas `accepts`
starts at the record's initial state. Product and its two named special cases
share one transition construction.

```ken
import Core.Classes.LawfulClasses (bool_and, bool_or, bool_not)

import Core.Logic.Transport (cong)

import Data.Collections.Derived (list_append)

pub data Dfa q a = MkDfa (q → a → q) q (q → Bool)

export MkDfa

pub fn complement (q : Type) (a : Type) (d : Dfa q a) : Dfa q a =
  MkDfa q a (step q a d) (start q a d) (λs. bool_not (final q a d s))

pub fn intersection
      (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a)
    : Dfa (Pair q r) a =
  product q r a bool_and d e

pub fn union (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a) : Dfa (Pair q r) a =
  product q r a bool_or d e

pub fn product
      (q : Type)
      (r : Type)
      (a : Type)
      (combine : Bool → Bool → Bool)
      (d : Dfa q a)
      (e : Dfa r a)
    : Dfa (Pair q r) a =
  MkDfa
    (Pair q r)
    a
    (λp. λx. mk_pair q r (step q a d (pair_fst q r p) x) (step r a e (pair_snd q r p) x))
    (mk_pair q r (start q a d) (start r a e))
    (λp. combine (final q a d (pair_fst q r p)) (final r a e (pair_snd q r p)))

pub fn accepts (q : Type) (a : Type) (d : Dfa q a) (w : List a) : Bool =
  final q a d (run q a d (start q a d) w)

pub fn run (q : Type) (a : Type) (d : Dfa q a) (s : q) (w : List a) : q =
  match w {
    Nil ↦ s;
    Cons x rest ↦ run q a d (step q a d s x) rest
  }

pub fn step (q : Type) (a : Type) (d : Dfa q a) (s : q) (x : a) : q =
  match d {
    MkDfa transition initial accepting ↦ transition s x
  }

pub fn start (q : Type) (a : Type) (d : Dfa q a) : q =
  match d {
    MkDfa transition initial accepting ↦ initial
  }

pub fn final (q : Type) (a : Type) (d : Dfa q a) (s : q) : Bool =
  match d {
    MkDfa transition initial accepting ↦ accepting s
  }
```

## 3. Using it

A machine with `Nat` states can count true symbols without comparing symbols
or states. The final predicate can distinguish the initial state from any
successor. Applying `complement` changes only that predicate; `intersection`
and `union` pair this machine's states with another machine's states.

```ken example
const seen_true : Dfa Nat Bool =
  MkDfa
    Nat
    Bool
    (λs.
      λx.
        match x {
          True ↦ Suc s;
          False ↦ s
        })
    Zero
    (λs.
      match s {
        Zero ↦ False;
        Suc n ↦ True
      })

theorem empty_is_rejected : Equal Bool (accepts Nat Bool seen_true (Nil Bool)) False = Proved

theorem true_is_accepted
    : Equal Bool (accepts Nat Bool seen_true (Cons Bool True (Nil Bool))) True =
  Proved
```

## 4. Laws & proofs

`accepts_complement` transports run-state equality through the negated final
predicate with `cong`; that equality is not generally reflexive on an abstract
record. `accepts_intersection` and `accepts_union` instantiate the general
`accepts_product` law, which transports a pair-of-runs equality through the
product's Boolean final predicate. Neither named operation needs its own
product recursion.

`run_complement` inducts over the word: complement changes the final predicate
but retains the transition. `run_product` inducts from arbitrary component
states, preserving the pair of runs for the *same* word. Its private empty-word
lemma exposes the pair-valued equality so that reflexivity checks without
relying on a dependent match arm's goal reduction. `run_append` inducts over
the first word from any initial state.

```ken
pub theorem accepts_complement
      (q : Type) (a : Type) (d : Dfa q a) (w : List a)
    : Equal Bool (accepts q a (complement q a d) w) (bool_not (accepts q a d w)) =
  cong
    q
    Bool
    (run q a (complement q a d) (start q a d) w)
    (run q a d (start q a d) w)
    (λs. bool_not (final q a d s))
    (run_complement q a d (start q a d) w)

pub theorem accepts_intersection
      (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a) (w : List a)
    : Equal Bool
        (accepts (Pair q r) a (intersection q r a d e) w)
        (bool_and (accepts q a d w) (accepts r a e w)) =
  accepts_product q r a bool_and d e w

pub theorem accepts_union
      (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a) (w : List a)
    : Equal Bool
        (accepts (Pair q r) a (union q r a d e) w)
        (bool_or (accepts q a d w) (accepts r a e w)) =
  accepts_product q r a bool_or d e w

pub theorem accepts_product
      (q : Type)
      (r : Type)
      (a : Type)
      (combine : Bool → Bool → Bool)
      (d : Dfa q a)
      (e : Dfa r a)
      (w : List a)
    : Equal Bool
        (accepts (Pair q r) a (product q r a combine d e) w)
        (combine (accepts q a d w) (accepts r a e w)) =
  cong
    (Pair q r)
    Bool
    (run (Pair q r) a (product q r a combine d e) (mk_pair q r (start q a d) (start r a e)) w)
    (mk_pair q r (run q a d (start q a d) w) (run r a e (start r a e) w))
    (final (Pair q r) a (product q r a combine d e))
    (run_product q r a combine d e (start q a d) (start r a e) w)

pub theorem run_complement
      (q : Type) (a : Type) (d : Dfa q a) (s : q) (w : List a)
    : Equal q (run q a (complement q a d) s w) (run q a d s w) =
  match w {
    Nil ↦ Refl;
    Cons x rest ↦ run_complement q a d (step q a d s x) rest
  }

pub theorem run_product
      (q : Type)
      (r : Type)
      (a : Type)
      (combine : Bool → Bool → Bool)
      (d : Dfa q a)
      (e : Dfa r a)
      (s1 : q)
      (s2 : r)
      (w : List a)
    : Equal
        (Pair q r)
        (run (Pair q r) a (product q r a combine d e) (mk_pair q r s1 s2) w)
        (mk_pair q r (run q a d s1 w) (run r a e s2 w)) =
  match w {
    Nil ↦ run_product_nil q r a combine d e s1 s2;
    Cons x rest ↦ run_product q r a combine d e (step q a d s1 x) (step r a e s2 x) rest
  }

theorem run_product_nil
      (q : Type)
      (r : Type)
      (a : Type)
      (combine : Bool → Bool → Bool)
      (d : Dfa q a)
      (e : Dfa r a)
      (s1 : q)
      (s2 : r)
    : Equal
        (Pair q r)
        (run (Pair q r) a (product q r a combine d e) (mk_pair q r s1 s2) (Nil a))
        (mk_pair q r s1 s2) =
  Refl

pub theorem run_append
      (q : Type) (a : Type) (d : Dfa q a) (s : q) (u : List a) (v : List a)
    : Equal q (run q a d s (list_append a u v)) (run q a d (run q a d s u) v) =
  match u {
    Nil ↦ Refl;
    Cons x rest ↦ run_append q a d (step q a d s x) rest v
  }
```

## 5. Design notes

A function-valued transition and an arbitrary state carrier keep `Dfa`
independent of finite-state enumeration or decidable equality. A
`Dfa (Fin n) a` becomes finite only with a separately supplied finiteness
certificate.
Decidable reachability, minimisation, regex derivatives, nondeterminism, and
byte-oriented lexer bridges are not operations of this package.

The general `product` also accepts Boolean combiners other than conjunction
and disjunction. Its predicate applies `combine` to the left machine's final
result **first** and to the right machine's result second; the order matters
for a noncommutative combiner.

## 6. References

- [Deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)
  — Wikipedia; introductory context for word processing and Boolean final
  states. This package's state carrier is not intrinsically finite.
- [Introduction to Automata Theory, Languages, and Computation](https://en.wikipedia.org/wiki/Introduction_to_Automata_Theory,_Languages,_and_Computation)
  — Hopcroft, Motwani, and Ullman; orientation to language operations and
  closure by automaton construction.

## 7. Trust & derivation

The contract is [formal languages §1](../../../../spec/50-stdlib/61-formal-languages.md).
The public API consists of `Dfa`, `MkDfa`, the nine operations in Definition,
and the seven theorems in Laws & proofs. The implementation uses the checked
prelude `List`, `Bool`, and `Pair` with `mk_pair` and its projections, plus
ordinary transparent imports from `Data.Collections.Derived`,
`Core.Classes.LawfulClasses`, and `Core.Logic.Transport`. It introduces no
primitive, Axiom, or local `trusted_base()` entry; inherited trust from
imported providers is not thereby erased.

| Reader task | Section |
|---|---|
| Construct a machine and combine languages | [Definition](#2-definition), [Using it](#3-using-it) |
| Check acceptance equations | [Laws & proofs](#4-laws--proofs) |
| Understand finiteness and trust | [Design notes](#5-design-notes), [Trust & derivation](#7-trust--derivation) |

`run_append` and `run_complement` are structural inductions; `run_product`
is a paired structural induction; `accepts_complement` and `accepts_product`
transport those equalities; intersection and union instantiate the general
product law. Acceptance and trust are also exercised by targeted package
tests. The code does not claim a finite-state decision procedure.
