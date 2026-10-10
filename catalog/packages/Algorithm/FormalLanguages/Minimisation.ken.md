# Minimisation — future-language equivalence and canonical DFA states

A finite certificate for both state carriers and the shared alphabet makes
DFA language equivalence decidable. Canonicalisation merges states with the
same future language inside the original carrier, without comparing states
for equality or promising a smallest carrier.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Two automata may recognize the same language while using different state
carriers or transition graphs. Comparing final-state Booleans along the same
input word expresses their language equality; a finite product search decides
whether a word reaches a disagreement. State equality is unnecessary.

A canonical state represents a *future language*, not an index in a new
quotient. `minimise` keeps the original carrier and routes transitions through
these representatives; its reduction guarantee concerns states reached from
its start, not all inhabitants of that carrier.

## 2. Definition

`same_future` is the independent all-words proposition. `equivalent` starts
at each machine's initial state; `equivalent_states` starts at the caller's
states. Both Boolean decisions search the product for a differing final
result. The private discriminator and its correctness proofs appear after
the public laws.

```ken
import Algorithm.FormalLanguages.Dfa
  (Dfa, MkDfa, final, start, run, step, accepts, product, run_product, run_append)

import Algorithm.FormalLanguages.Reachability
  (is_empty, reachable, find_word, find_word_sound, find_word_complete)

import Data.Finite.Finite (Finite, elements, covers, pair_finite)

import Data.Collections.Derived (list_elem, list_append)

import Data.Sums.Combinators (is_some)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (cong, sym, trans)

import Core.Classes.LawfulClasses (bool_eq, bool_not)

pub fn same_future
      (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a) (s : q) (t : r)
    : Omega =
  (w : List a) → Equal Bool (final q a d (run q a d s w)) (final r a e (run r a e t w))

pub fn equivalent
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
    : Bool =
  equivalent_states q r a fq fr fa d e (start q a d) (start r a e)

pub fn equivalent_states
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
    : Bool =
  let disagreement =
    product q r a disagree d e
  in
    bool_not
      (reachable
        (Pair q r)
        a
        (pair_finite q r fq fr)
        fa
        disagreement
        (final (Pair q r) a disagreement)
        (mk_pair q r s t))
```

The minimised machine starts at a canonical representative and canonicalises
every successor, retaining the original final predicate and state carrier.
The choice of representative is private.

```ken
pub fn minimise (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) : Dfa q a =
  MkDfa
    q
    a
    (λs. λx. canonical q a fq fa d (step q a d s x))
    (canonical q a fq fa d (start q a d))
    (final q a d)

pub fn canonical
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (s : q)
    : q =
  pick q a fq fa d (elements q fq) s
```

## 3. Using it

An automaton with two toggling states can accept every word just like a
single-state machine. Only the Boolean outcome and language are public; the
example deliberately does not assert which state is selected as canonical.
`only_empty` distinguishes a disagreement reachable after a transition.

```ken example
import Data.Finite.Finite (bool_finite, unit_finite)

const accept_all : Dfa Unit Bool = MkDfa Unit Bool (λs. λx. s) MkUnit (λs. True)

const twin : Dfa Bool Bool = MkDfa Bool Bool (λs. λx. bool_not s) True (λs. True)

const only_empty : Dfa Bool Bool = MkDfa Bool Bool (λs. λx. False) True (λs. s)

theorem twin_equivalent_all
    : Equal Bool
        (equivalent Bool Unit Bool bool_finite unit_finite bool_finite twin accept_all)
        True =
  Proved

theorem only_empty_not_all
    : Equal Bool
        (equivalent Bool Unit Bool bool_finite unit_finite bool_finite only_empty accept_all)
        False =
  Proved

theorem twin_states_equivalent
    : Equal Bool
        (equivalent_states
          Bool
          Bool
          Bool
          bool_finite
          bool_finite
          bool_finite
          twin
          twin
          True
          False)
        True =
  Proved

theorem only_empty_states_differ
    : Equal Bool
        (equivalent_states
          Bool
          Bool
          Bool
          bool_finite
          bool_finite
          bool_finite
          only_empty
          only_empty
          True
          False)
        False =
  Proved

theorem twin_original_moves
    : Equal Bool (run Bool Bool twin True (Cons Bool False (Nil Bool))) False =
  Proved

theorem twin_minimised_accepts
    : Equal Bool
        (accepts
          Bool
          Bool
          (minimise Bool Bool bool_finite bool_finite twin)
          (Cons Bool False (Nil Bool)))
        True =
  Proved

theorem only_empty_minimised_rejects
    : Equal Bool
        (accepts
          Bool
          Bool
          (minimise Bool Bool bool_finite bool_finite only_empty)
          (Cons Bool False (Nil Bool)))
        False =
  Proved
```

## 4. Laws & proofs

The four equivalence laws prove both directions for whole machines and
arbitrary supplied states. Soundness uses `find_word_complete` to rule out a
reachable disagreement; completeness uses `find_word_sound` to refute any
returned disagreement witness when all futures agree. `run_product`
connects the product's final predicate to the separate component runs.

```ken
pub theorem equivalent_sound
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
    : Equal Bool (equivalent q r a fq fr fa d e) True
      → (w : List a)
      → Equal Bool (accepts q a d w) (accepts r a e w) =
  equivalent_states_sound q r a fq fr fa d e (start q a d) (start r a e)

pub theorem equivalent_complete
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
    : ((w : List a) → Equal Bool (accepts q a d w) (accepts r a e w))
      → Equal Bool (equivalent q r a fq fr fa d e) True =
  equivalent_states_complete q r a fq fr fa d e (start q a d) (start r a e)

pub theorem equivalent_states_sound
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
    : Equal Bool (equivalent_states q r a fq fr fa d e s t) True → same_future q r a d e s t =
  λh.
    λw.
      sound_at
        q
        r
        a
        fq
        fr
        fa
        d
        e
        s
        t
        w
        (final
          (Pair q r)
          a
          (product q r a disagree d e)
          (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
        Refl
        h

pub theorem equivalent_states_complete
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
    : same_future q r a d e s t → Equal Bool (equivalent_states q r a fq fr fa d e s t) True =
  λagree.
    complete_at
      q
      r
      a
      fq
      fr
      fa
      d
      e
      s
      t
      agree
      (find_word
        (Pair q r)
        a
        (pair_finite q r fq fr)
        fa
        (product q r a disagree d e)
        (final (Pair q r) a (product q r a disagree d e))
        (mk_pair q r s t))
      Refl
```

The other four laws show that canonicalisation preserves every future,
equal futures have equal representatives, the minimised machine accepts the
original language, and equal future languages at reached states identify
the *same* state. No law claims a minimum state count or a new quotient.

```ken
pub theorem canonical_same_future
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (s : q)
    : same_future q q a d d (canonical q a fq fa d s) s =
  pick_same_future q a fq fa d (elements q fq) s

pub theorem canonical_unique
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (s : q) (t : q)
    : same_future q q a d d s t → Equal q (canonical q a fq fa d s) (canonical q a fq fa d t) =
  λsim. pick_unique q a fq fa d (elements q fq) s t sim (covers q fq s)

pub theorem accepts_minimise
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (w : List a)
    : Equal Bool (accepts q a (minimise q a fq fa d) w) (accepts q a d w) =
  trans
    Bool
    (accepts q a (minimise q a fq fa d) w)
    (final q a d (run q a d (canonical q a fq fa d (start q a d)) w))
    (accepts q a d w)
    (run_minimise q a fq fa d (canonical q a fq fa d (start q a d)) w)
    (canonical_same_future q a fq fa d (start q a d) w)

pub theorem minimise_reduced
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (u : List a)
      (v : List a)
    : ((w : List a)
        → Equal
        Bool
        (accepts q a (minimise q a fq fa d) (list_append a u w))
        (accepts q a (minimise q a fq fa d) (list_append a v w)))
      → Equal q
        (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u)
        (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) v) =
  λresidual.
    let
      m = minimise q a fq fa d;
      su = run q a m (start q a m) u;
      sv = run q a m (start q a m) v
    in
      trans
        q
        su
        (canonical q a fq fa d su)
        sv
        (sym q (canonical q a fq fa d su) su (reached_canonical q a fq fa d (start q a d) u))
        (trans
          q
          (canonical q a fq fa d su)
          (canonical q a fq fa d sv)
          sv
          (canonical_unique
            q
            a
            fq
            fa
            d
            su
            sv
            (λw.
              trans
                Bool
                (final q a d (run q a d su w))
                (accepts q a m (list_append a u w))
                (final q a d (run q a d sv w))
                (sym
                  Bool
                  (accepts q a m (list_append a u w))
                  (final q a d (run q a d su w))
                  (residual_state q a fq fa d u w))
                (trans
                  Bool
                  (accepts q a m (list_append a u w))
                  (accepts q a m (list_append a v w))
                  (final q a d (run q a d sv w))
                  (residual w)
                  (residual_state q a fq fa d v w))))
          (reached_canonical q a fq fa d (start q a d) v))
```

Private helpers below discharge those laws. Computed `Bool` and `Option`
case analyses take the computed result together with its equality in an
`_at` lemma. `pick` walks the supplied enumeration; coverage permits the
uniqueness proof even when the enumeration contains duplicates.

```ken
fn disagree (b : Bool) (c : Bool) : Bool = bool_not (bool_eq b c)

theorem product_final
      (q : Type) (r : Type) (a : Type) (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) (w : List a)
    : Equal Bool
        (final
          (Pair q r)
          a
          (product q r a disagree d e)
          (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
        (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w))) =
  cong
    (Pair q r)
    Bool
    (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w)
    (mk_pair q r (run q a d s w) (run r a e t w))
    (final (Pair q r) a (product q r a disagree d e))
    (run_product q r a disagree d e s t w)

theorem sound_at
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
      (w : List a)
      (b : Bool)
    : Equal Bool
        (final
          (Pair q r)
          a
          (product q r a disagree d e)
          (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
        b
      → Equal Bool (equivalent_states q r a fq fr fa d e s t) True
      → Equal Bool (final q a d (run q a d s w)) (final r a e (run r a e t w)) =
  match b {
    True ↦
      λhb.
        λh.
          let found =
            reachable
              (Pair q r)
              a
              (pair_finite q r fq fr)
              fa
              (product q r a disagree d e)
              (final (Pair q r) a (product q r a disagree d e))
              (mk_pair q r s t)
          in
            absurd
              (trans
                Bool
                (bool_not True)
                (bool_not found)
                True
                (cong
                  Bool
                  Bool
                  True
                  found
                  bool_not
                  (sym
                    Bool
                    found
                    True
                    (find_word_complete
                      (Pair q r)
                      a
                      (pair_finite q r fq fr)
                      fa
                      (product q r a disagree d e)
                      (final (Pair q r) a (product q r a disagree d e))
                      (mk_pair q r s t)
                      w
                      hb)))
                h);
    False ↦
      λhb.
        λh.
          agree_of_not_disagree
            (final q a d (run q a d s w))
            (final r a e (run r a e t w))
            (trans
              Bool
              (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w)))
              (final
                (Pair q r)
                a
                (product q r a disagree d e)
                (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
              False
              (sym
                Bool
                (final
                  (Pair q r)
                  a
                  (product q r a disagree d e)
                  (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
                (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w)))
                (product_final q r a d e s t w))
              hb)
  }

theorem complete_at
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
      (agree : same_future q r a d e s t)
      (o : Option (List a))
    : Equal
        (Option (List a))
        (find_word
          (Pair q r)
          a
          (pair_finite q r fq fr)
          fa
          (product q r a disagree d e)
          (final (Pair q r) a (product q r a disagree d e))
          (mk_pair q r s t))
        o
      → Equal Bool (equivalent_states q r a fq fr fa d e s t) True =
  λfound.
    trans
      Bool
      (equivalent_states q r a fq fr fa d e s t)
      (bool_not (is_some (List a) o))
      True
      (cong
        (Option (List a))
        Bool
        (find_word
          (Pair q r)
          a
          (pair_finite q r fq fr)
          fa
          (product q r a disagree d e)
          (final (Pair q r) a (product q r a disagree d e))
          (mk_pair q r s t))
        o
        (λx. bool_not (is_some (List a) x))
        found)
      (no_witness q r a fq fr fa d e s t agree o found)

theorem no_witness
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
      (s : q)
      (t : r)
      (agree : same_future q r a d e s t)
      (o : Option (List a))
    : Equal
        (Option (List a))
        (find_word
          (Pair q r)
          a
          (pair_finite q r fq fr)
          fa
          (product q r a disagree d e)
          (final (Pair q r) a (product q r a disagree d e))
          (mk_pair q r s t))
        o
      → Equal Bool (bool_not (is_some (List a) o)) True =
  match o {
    None ↦ λfound. Proved;
    Some w ↦
      λfound.
        absurd
          (trans
            Bool
            True
            (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w)))
            False
            (trans
              Bool
              True
              (final
                (Pair q r)
                a
                (product q r a disagree d e)
                (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
              (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w)))
              (sym
                Bool
                (final
                  (Pair q r)
                  a
                  (product q r a disagree d e)
                  (run (Pair q r) a (product q r a disagree d e) (mk_pair q r s t) w))
                True
                (find_word_sound
                  (Pair q r)
                  a
                  (pair_finite q r fq fr)
                  fa
                  (product q r a disagree d e)
                  (final (Pair q r) a (product q r a disagree d e))
                  (mk_pair q r s t)
                  w
                  found))
              (product_final q r a d e s t w))
            (trans
              Bool
              (disagree (final q a d (run q a d s w)) (final r a e (run r a e t w)))
              (disagree (final r a e (run r a e t w)) (final r a e (run r a e t w)))
              False
              (cong
                Bool
                Bool
                (final q a d (run q a d s w))
                (final r a e (run r a e t w))
                (λb. disagree b (final r a e (run r a e t w)))
                (agree w))
              (disagree_self (final r a e (run r a e t w)))))
  }

theorem agree_of_not_disagree
      (b : Bool) (c : Bool)
    : Equal Bool (disagree b c) False → Equal Bool b c =
  match b {
    True ↦
      match c {
        True ↦ λh. Proved;
        False ↦ λh. absurd h
      };
    False ↦
      match c {
        True ↦ λh. absurd h;
        False ↦ λh. Proved
      }
  }

theorem disagree_self (b : Bool) : Equal Bool (disagree b b) False =
  match b {
    True ↦ Proved;
    False ↦ Proved
  }

theorem residual_state
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (u : List a)
      (w : List a)
    : Equal Bool
        (accepts q a (minimise q a fq fa d) (list_append a u w))
        (final
          q
          a
          d
          (run q a d (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u) w)) =
  trans
    Bool
    (accepts q a (minimise q a fq fa d) (list_append a u w))
    (final
      q
      a
      d
      (run
        q
        a
        (minimise q a fq fa d)
        (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u)
        w))
    (final
      q
      a
      d
      (run q a d (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u) w))
    (cong
      q
      Bool
      (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) (list_append a u w))
      (run
        q
        a
        (minimise q a fq fa d)
        (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u)
        w)
      (final q a d)
      (run_append q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u w))
    (run_minimise
      q
      a
      fq
      fa
      d
      (run q a (minimise q a fq fa d) (start q a (minimise q a fq fa d)) u)
      w)

theorem run_minimise
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (s : q) (w : List a)
    : Equal Bool
        (final q a d (run q a (minimise q a fq fa d) s w))
        (final q a d (run q a d s w)) =
  match w {
    Nil ↦ Refl;
    Cons x rest ↦
      trans
        Bool
        (final
          q
          a
          d
          (run q a (minimise q a fq fa d) (canonical q a fq fa d (step q a d s x)) rest))
        (final q a d (run q a d (canonical q a fq fa d (step q a d s x)) rest))
        (final q a d (run q a d (step q a d s x) rest))
        (run_minimise q a fq fa d (canonical q a fq fa d (step q a d s x)) rest)
        (canonical_same_future q a fq fa d (step q a d s x) rest)
  }

theorem reached_canonical
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (s : q) (u : List a)
    : Equal q
        (canonical q a fq fa d (run q a (minimise q a fq fa d) (canonical q a fq fa d s) u))
        (run q a (minimise q a fq fa d) (canonical q a fq fa d s) u) =
  match u {
    Nil ↦
      canonical_unique
        q
        a
        fq
        fa
        d
        (canonical q a fq fa d s)
        s
        (canonical_same_future q a fq fa d s);
    Cons x rest ↦ reached_canonical q a fq fa d (step q a d (canonical q a fq fa d s) x) rest
  }

fn pick
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (es : List q) (s : q)
    : q =
  match es {
    Nil ↦ s;
    Cons e rest ↦
      choose q (equivalent_states q q a fq fq fa d d e s) e (pick q a fq fa d rest s)
  }

fn choose (q : Type) (b : Bool) (x : q) (y : q) : q =
  match b {
    True ↦ x;
    False ↦ y
  }

theorem pick_same_future
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (es : List q) (s : q)
    : same_future q q a d d (pick q a fq fa d es s) s =
  match es {
    Nil ↦ λw. Refl;
    Cons e rest ↦
      pick_same_future_at q a fq fa d e rest s (equivalent_states q q a fq fq fa d d e s) Refl
  }

theorem pick_same_future_at
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (e : q)
      (rest : List q)
      (s : q)
      (b : Bool)
    : Equal Bool (equivalent_states q q a fq fq fa d d e s) b
      → same_future q q a d d (choose q b e (pick q a fq fa d rest s)) s =
  match b {
    True ↦ λh. equivalent_states_sound q q a fq fq fa d d e s h;
    False ↦ λh. pick_same_future q a fq fa d rest s
  }

theorem pick_unique
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (es : List q)
      (s : q)
      (t : q)
    : same_future q q a d d s t
      → list_elem q s es
      → Equal q (pick q a fq fa d es s) (pick q a fq fa d es t) =
  match es {
    Nil ↦ λsim. λmember. absurd member;
    Cons e rest ↦
      λsim.
        λmember.
          pick_unique_at
            q
            a
            fq
            fa
            d
            e
            rest
            s
            t
            sim
            member
            (equivalent_states q q a fq fq fa d d e s)
            (equivalent_states q q a fq fq fa d d e t)
            Refl
            Refl
  }

theorem pick_unique_at
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (e : q)
      (rest : List q)
      (s : q)
      (t : q)
      (sim : same_future q q a d d s t)
      (member : list_elem q s (Cons q e rest))
      (b : Bool)
      (c : Bool)
    : Equal Bool (equivalent_states q q a fq fq fa d d e s) b
      → Equal Bool (equivalent_states q q a fq fq fa d d e t) c
      → Equal q
        (choose q b e (pick q a fq fa d rest s))
        (choose q c e (pick q a fq fa d rest t)) =
  match b {
    True ↦
      match c {
        True ↦ λhb. λhc. Refl;
        False ↦
          λhb.
            λhc.
              absurd
                (trans
                  Bool
                  True
                  (equivalent_states q q a fq fq fa d d e t)
                  False
                  (sym
                    Bool
                    (equivalent_states q q a fq fq fa d d e t)
                    True
                    (equivalent_states_complete
                      q
                      q
                      a
                      fq
                      fq
                      fa
                      d
                      d
                      e
                      t
                      (λw.
                        trans
                          Bool
                          (final q a d (run q a d e w))
                          (final q a d (run q a d s w))
                          (final q a d (run q a d t w))
                          (equivalent_states_sound q q a fq fq fa d d e s hb w)
                          (sim w))))
                  hc)
      };
    False ↦
      match c {
        True ↦
          λhb.
            λhc.
              absurd
                (trans
                  Bool
                  True
                  (equivalent_states q q a fq fq fa d d e s)
                  False
                  (sym
                    Bool
                    (equivalent_states q q a fq fq fa d d e s)
                    True
                    (equivalent_states_complete
                      q
                      q
                      a
                      fq
                      fq
                      fa
                      d
                      d
                      e
                      s
                      (λw.
                        trans
                          Bool
                          (final q a d (run q a d e w))
                          (final q a d (run q a d t w))
                          (final q a d (run q a d s w))
                          (equivalent_states_sound q q a fq fq fa d d e t hc w)
                          (sym
                            Bool
                            (final q a d (run q a d s w))
                            (final q a d (run q a d t w))
                            (sim w)))))
                  hb);
        False ↦
          λhb.
            λhc.
              elim_trunc
                (Equal q (pick q a fq fa d rest s) (pick q a fq fa d rest t))
                (λcase.
                  match case {
                    Inl same ↦
                      absurd
                        (trans
                          Bool
                          True
                          (equivalent_states q q a fq fq fa d d e s)
                          False
                          (sym
                            Bool
                            (equivalent_states q q a fq fq fa d d e s)
                            True
                            (equivalent_states_complete
                              q
                              q
                              a
                              fq
                              fq
                              fa
                              d
                              d
                              e
                              s
                              (λw.
                                cong
                                  q
                                  Bool
                                  e
                                  s
                                  (λx. final q a d (run q a d x w))
                                  (sym q s e same))))
                          hb);
                    Inr later ↦ pick_unique q a fq fa d rest s t sim later
                  })
                member
      }
  }

theorem equivalent_is_empty_view
      (q : Type)
      (r : Type)
      (a : Type)
      (fq : Finite q)
      (fr : Finite r)
      (fa : Finite a)
      (d : Dfa q a)
      (e : Dfa r a)
    : Equal Bool
        (equivalent q r a fq fr fa d e)
        (is_empty (Pair q r) a (pair_finite q r fq fr) fa (product q r a disagree d e)) =
  Refl
```

## 5. Design notes

The disagreement machine is `product disagree d e`, certified finite by
`pair_finite fq fr`. The alphabet certificate lets `reachable` search all
input symbols. Neither certificate is a `DecEq` dictionary. In particular,
`canonical` selects the first equivalent listed state but the contract does
not expose a representative or an enumeration order. Its fallback preserves
the future even when the supplied candidate list is empty; full coverage
underwrites equality of choices for equivalent states.

The transition of `minimise` canonicalises successors but its final predicate
is the original one. Its reachable states are canonical fixed points;
unreachable inhabitants of `q` may still be redundant. A state-count claim
would need distinctness and cardinality machinery beyond this package.

## 6. References

- [DFA minimization](https://en.wikipedia.org/wiki/DFA_minimization) —
  Wikipedia; introductory account of identifying states by their future
  languages. This package makes no algorithmic complexity claim.
- [Myhill–Nerode theorem](https://en.wikipedia.org/wiki/Myhill%E2%80%93Nerode_theorem)
  — Wikipedia; context for distinguishing residual languages and the
  reachable-state equality in `minimise_reduced`.
- [Introduction to Automata Theory, Languages, and Computation](https://en.wikipedia.org/wiki/Introduction_to_Automata_Theory,_Languages,_and_Computation)
  — Hopcroft, Motwani, and Ullman; language-equivalence background.

## 7. Trust & derivation

The contract is
[formal languages §5](../../../../spec/50-stdlib/61-formal-languages.md)
and its [seed](../../../../conformance/stdlib/formal-languages/seed-minimisation.md).
This entry delivers exactly five public operations (`same_future`, `equivalent`,
`equivalent_states`, `canonical`, `minimise`) and eight public checked laws
named above. All other declarations are private, including the disagreement
predicate and the enumeration-based representative choice.

| Reader task | Section |
|---|---|
| Decide whole-machine or state-specific language equality | [Definition](#2-definition), [Using it](#3-using-it) |
| Inspect the eight checked guarantees | [Laws & proofs](#4-laws--proofs) |
| Check the reachability mechanism and trust boundary | [Design notes](#5-design-notes), [Trust & derivation](#7-trust--derivation) |

The derivation uses the checked `Dfa` product and `run` laws, finite pair
coverage, the complete reachability decision, and ordinary `Bool`, `List`,
`Pair`, `Equal`, and `Option` operations. `cong`, `sym`, and `trans` transport
checked equalities; `Or` and truncated membership carry coverage proofs.
All eight public laws have kernel-checked proof terms. The package introduces
no primitive, `Axiom`, postulate, foreign declaration, or local
`trusted_base()` entry; this does not erase trust inherited from imports.
Targeted acceptance checks exercise roots loading, generic law consumers,
closed seed behavior, and the exact trust delta.
