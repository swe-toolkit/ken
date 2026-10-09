# Reachability and emptiness of a finite-state automaton

With explicit finite certificates for both states and symbols, reachability
can return a witness word or report that no target state can be reached.
The emptiness decision is a view of this search, not a second algorithm.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

An automaton's total transition says how it processes a finite word; it does
not alone give a finite search space. A certificate for its state type and
another for its input alphabet supply both enumerations without deciding
state equality or imposing an enumeration order.

## 2. Definition

`is_empty` negates the Boolean view of `find_word`. `accepted_word` searches
for a final state from the machine's start. The general `find_word` and
`reachable` accept any Boolean state predicate and starting state.

```ken
import Algorithm.FormalLanguages.Dfa (Dfa, final, start, run, step, accepts)

import Data.Finite.Finite (Finite, elements, covers)

import Data.Collections.Derived (list_elem)

import Data.Collections.List (length)

import Data.Sums.Combinators (is_some)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (cong, sym, trans)

import Core.Classes.LawfulClasses (bool_or, bool_not, leq_nat, or_left, or_right)

import Data.Numeric.Nat.Order (leq_nat_weaken_right)

pub fn is_empty (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) : Bool =
  bool_not (reachable q a fq fa d (final q a d) (start q a d))

pub fn accepted_word
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a)
    : Option (List a) =
  find_word q a fq fa d (final q a d) (start q a d)

pub fn reachable
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
    : Bool =
  is_some (List a) (find_word q a fq fa d target s)

pub fn find_word
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
    : Option (List a) =
  search q a (elements a fa) d target (length q (elements q fq)) s
```

## 3. Using it

Supply both the state and alphabet certificates to ask for a witness. A
`None` result means no finite word reaches the requested predicate. For an
intersection, compose the state certificates with `pair_finite`; the same
alphabet certificate applies to both machines. This one-state automaton has
no final state, so its language is empty.

```ken example
import Data.Vector.Vector (Fin, FZero)

import Data.Finite.Finite (fin_finite)

import Algorithm.FormalLanguages.Dfa (MkDfa)

const rejecting : Dfa (Fin (Suc Zero)) (Fin (Suc Zero)) =
  MkDfa (Fin (Suc Zero)) (Fin (Suc Zero)) (λstate. λsymbol. state) (FZero Zero) (λstate. False)

const rejecting_is_empty : Bool =
  is_empty
    (Fin (Suc Zero))
    (Fin (Suc Zero))
    (fin_finite (Suc Zero))
    (fin_finite (Suc Zero))
    rejecting

theorem rejecting_empty : Equal Bool rejecting_is_empty True = Proved
```

## 4. Laws & proofs

`is_empty_rejects` and `accepted_word_accepts` connect the two accepted-
language views to the general sound and complete `find_word` laws. Soundness
follows the word returned by the search. Completeness counts Boolean
reachability over the certified state enumeration: without a stable step the
count grows, yet it cannot exceed the enumeration length even with duplicates.
Alphabet coverage relates an arbitrary witness word to the symbols visited by
search. Boolean reachability steps reuse the public `or_left` and
`or_right` lemmas from `Core.Classes.LawfulClasses`; the search helpers remain
private to the package.

```ken
pub theorem is_empty_rejects
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (w : List a)
    : Equal Bool (is_empty q a fq fa d) True → Equal Bool (accepts q a d w) False =
  is_empty_rejects_at q a fq fa d w (accepts q a d w) Refl

theorem is_empty_rejects_at
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (w : List a)
      (b : Bool)
    : Equal Bool (accepts q a d w) b
      → Equal Bool (is_empty q a fq fa d) True
      → Equal Bool (accepts q a d w) False =
  match b {
    True ↦
      λh.
        λempty.
          let accepting_state_reachable =
            reachable q a fq fa d (final q a d) (start q a d)
          in
            absurd
              (trans
                Bool
                (bool_not True)
                (bool_not accepting_state_reachable)
                True
                (cong
                  Bool
                  Bool
                  True
                  accepting_state_reachable
                  bool_not
                  (sym
                    Bool
                    accepting_state_reachable
                    True
                    (find_word_complete q a fq fa d (final q a d) (start q a d) w h)))
                empty);
    False ↦ λh. λempty. h
  }

pub theorem accepted_word_accepts
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (d : Dfa q a) (w : List a)
    : Equal (Option (List a)) (accepted_word q a fq fa d) (Some (List a) w)
      → Equal Bool (accepts q a d w) True =
  find_word_sound q a fq fa d (final q a d) (start q a d) w

pub theorem find_word_complete
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
      (w : List a)
    : Equal Bool (target (run q a d s w)) True
      → Equal Bool (reachable q a fq fa d target s) True =
  λh.
    let
      symbols = elements a fa;
      states = elements q fq;
      fuel_bound = length q states
    in
      trans
        Bool
        (reachable q a fq fa d target s)
        (reach q a symbols d target fuel_bound s)
        True
        (search_reach q a symbols d target fuel_bound s)
        (reach_complete q a symbols (covers a fa) states (covers q fq) d target s w h)

pub theorem find_word_sound
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (fa : Finite a)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
      (w : List a)
    : Equal (Option (List a)) (find_word q a fq fa d target s) (Some (List a) w)
      → Equal Bool (target (run q a d s w)) True =
  search_sound q a (elements a fa) d target (length q (elements q fq)) s w

fn search
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (fuel : Nat)
      (s : q)
    : Option (List a) =
  search_at q a symbols d target fuel s (target s)

fn search_at
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (fuel : Nat)
      (s : q)
      (here : Bool)
    : Option (List a) =
  match here {
    True ↦ Some (List a) (Nil a);
    False ↦
      match fuel {
        Zero ↦ None (List a);
        Suc k ↦ first_word a (λx. search q a symbols d target k (step q a d s x)) symbols
      }
  }

fn first_word (a : Type) (f : a → Option (List a)) (xs : List a) : Option (List a) =
  match xs {
    Nil ↦ None (List a);
    Cons x rest ↦ first_word_at a f x rest (f x)
  }

fn first_word_at
      (a : Type) (f : a → Option (List a)) (x : a) (rest : List a) (found : Option (List a))
    : Option (List a) =
  match found {
    Some w ↦ Some (List a) (Cons a x w);
    None ↦ first_word a f rest
  }

fn hits
      (q : Type) (a : Type) (d : Dfa q a) (target : q → Bool) (s : q) (found : Option (List a))
    : Omega =
  match found {
    None ↦ Equal Bool True True;
    Some w ↦ Equal Bool (target (run q a d s w)) True
  }

theorem search_sound
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (fuel : Nat)
      (s : q)
      (w : List a)
    : Equal (Option (List a)) (search q a symbols d target fuel s) (Some (List a) w)
      → Equal Bool (target (run q a d s w)) True =
  λfound. J (λo _. hits q a d target s o) (search_hits q a symbols d target fuel s) found

theorem search_hits
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (fuel : Nat)
      (s : q)
    : hits q a d target s (search q a symbols d target fuel s) =
  search_at_hits q a symbols d target fuel s (target s) Refl

theorem search_at_hits
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (fuel : Nat)
      (s : q)
      (here : Bool)
    : Equal Bool (target s) here
      → hits q a d target s (search_at q a symbols d target fuel s here) =
  match here {
    True ↦ λh. h;
    False ↦
      λh.
        match fuel {
          Zero ↦ Proved;
          Suc k ↦
            first_word_hits
              q
              a
              d
              target
              s
              (λx. search q a symbols d target k (step q a d s x))
              (λx. search_hits q a symbols d target k (step q a d s x))
              symbols
        }
  }

theorem first_word_hits
      (q : Type)
      (a : Type)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
      (f : a → Option (List a))
      (each : (x : a) → hits q a d target (step q a d s x) (f x))
      (xs : List a)
    : hits q a d target s (first_word a f xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      first_word_at_hits
        q
        a
        d
        target
        s
        f
        x
        rest
        (first_word_hits q a d target s f each rest)
        (f x)
        (each x)
  }

theorem first_word_at_hits
      (q : Type)
      (a : Type)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
      (f : a → Option (List a))
      (x : a)
      (rest : List a)
      (later : hits q a d target s (first_word a f rest))
      (found : Option (List a))
    : hits q a d target (step q a d s x) found
      → hits q a d target s (first_word_at a f x rest found) =
  match found {
    None ↦ λnow. later;
    Some w ↦ λnow. now
  }

theorem search_reach
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (k : Nat)
      (s : q)
    : Equal Bool
        (is_some (List a) (search q a symbols d target k s))
        (reach q a symbols d target k s) =
  search_at_reach q a symbols d target k s (target s) Refl

theorem search_at_reach
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (k : Nat)
      (s : q)
      (here : Bool)
    : Equal Bool (target s) here
      → Equal Bool
        (is_some (List a) (search_at q a symbols d target k s here))
        (reach q a symbols d target k s) =
  match here {
    True ↦
      λh.
        match k {
          Zero ↦ sym Bool (target s) True h;
          Suc j ↦
            cong
              Bool
              Bool
              True
              (target s)
              (λb.
                bool_or
                  b
                  (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols))
              (sym Bool (target s) True h)
        };
    False ↦
      λh.
        match k {
          Zero ↦ sym Bool (target s) False h;
          Suc j ↦
            trans
              Bool
              (is_some
                (List a)
                (first_word a (λx. search q a symbols d target j (step q a d s x)) symbols))
              (any_of
                a
                (λx. is_some (List a) (search q a symbols d target j (step q a d s x)))
                symbols)
              (bool_or
                (target s)
                (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols))
              (first_word_some a (λx. search q a symbols d target j (step q a d s x)) symbols)
              (trans
                Bool
                (any_of
                  a
                  (λx. is_some (List a) (search q a symbols d target j (step q a d s x)))
                  symbols)
                (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols)
                (bool_or
                  (target s)
                  (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols))
                (any_of_ext
                  a
                  (λx. is_some (List a) (search q a symbols d target j (step q a d s x)))
                  (λx. reach q a symbols d target j (step q a d s x))
                  (λx. search_reach q a symbols d target j (step q a d s x))
                  symbols)
                (cong
                  Bool
                  Bool
                  False
                  (target s)
                  (λb.
                    bool_or
                      b
                      (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols))
                  (sym Bool (target s) False h)))
        }
  }

theorem first_word_some
      (a : Type) (f : a → Option (List a)) (xs : List a)
    : Equal Bool (is_some (List a) (first_word a f xs)) (any_of a (some_at a f) xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      trans
        Bool
        (is_some (List a) (first_word_at a f x rest (f x)))
        (bool_or (is_some (List a) (f x)) (is_some (List a) (first_word a f rest)))
        (bool_or (is_some (List a) (f x)) (any_of a (some_at a f) rest))
        (first_word_at_some a f x rest (f x))
        (cong
          Bool
          Bool
          (is_some (List a) (first_word a f rest))
          (any_of a (some_at a f) rest)
          (λc. bool_or (is_some (List a) (f x)) c)
          (first_word_some a f rest))
  }

theorem first_word_at_some
      (a : Type) (f : a → Option (List a)) (x : a) (rest : List a) (found : Option (List a))
    : Equal Bool
        (is_some (List a) (first_word_at a f x rest found))
        (bool_or (is_some (List a) found) (is_some (List a) (first_word a f rest))) =
  match found {
    None ↦ first_word_at_none a f x rest;
    Some w ↦ first_word_at_found a f x rest w
  }

theorem first_word_at_none
      (a : Type) (f : a → Option (List a)) (x : a) (rest : List a)
    : Eq Bool
        (is_some (List a) (first_word_at a f x rest (None (List a))))
        (bool_or (is_some (List a) (None (List a))) (is_some (List a) (first_word a f rest))) =
  Refl

theorem first_word_at_found
      (a : Type) (f : a → Option (List a)) (x : a) (rest : List a) (w : List a)
    : Eq Bool
        (is_some (List a) (first_word_at a f x rest (Some (List a) w)))
        (bool_or
          (is_some (List a) (Some (List a) w))
          (is_some (List a) (first_word a f rest))) =
  Proved

fn some_at (a : Type) (f : a → Option (List a)) (x : a) : Bool = is_some (List a) (f x)

theorem reach_complete
      (q : Type)
      (a : Type)
      (symbols : List a)
      (symbol_cover : (x : a) → list_elem a x symbols)
      (states : List q)
      (cover : (x : q) → list_elem q x states)
      (d : Dfa q a)
      (target : q → Bool)
      (s : q)
      (w : List a)
    : Equal Bool (target (run q a d s w)) True
      → Equal Bool (reach q a symbols d target (length q states) s) True =
  λh.
    stable_closed
      q
      a
      symbols
      d
      target
      (length q states)
      (stable_at_bound q a symbols d target states cover)
      (length a w)
      s
      (reach_walk q a symbols symbol_cover d target w s h)

theorem stable_at_bound
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (states : List q)
      (cover : (x : q) → list_elem q x states)
    : stable q a symbols d target (length q states) =
  match stable_or_grown q a symbols d target states cover (length q states) {
    Inl st ↦ st;
    Inr ge ↦
      match growth q a symbols d target states cover (length q states) ge {
        Inl st ↦ st;
        Inr grown ↦
          leq_suc_self_absurd
            (length q states)
            (stable q a symbols d target (length q states))
            ((proof trans for leq_nat)
              (Suc (length q states))
              (reach_count q a symbols d target states (Suc (length q states)))
              (length q states)
              grown
              (count_bound q (reach q a symbols d target (Suc (length q states))) states))
      }
  }

fn stable_or_grown
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (states : List q)
      (cover : (x : q) → list_elem q x states)
      (k : Nat)
    : Or
        (stable q a symbols d target k)
        (Equal Bool (leq_nat k (reach_count q a symbols d target states k)) True) =
  match k {
    Zero ↦
      Inr
        (stable q a symbols d target Zero)
        (Equal Bool (leq_nat Zero (reach_count q a symbols d target states Zero)) True)
        Proved;
    Suc j ↦
      match stable_or_grown q a symbols d target states cover j {
        Inl st ↦
          Inl
            (stable q a symbols d target (Suc j))
            (Equal
              Bool
              (leq_nat (Suc j) (reach_count q a symbols d target states (Suc j)))
              True)
            (stable_next q a symbols d target j st);
        Inr ge ↦
          match growth q a symbols d target states cover j ge {
            Inl st ↦
              Inl
                (stable q a symbols d target (Suc j))
                (Equal
                  Bool
                  (leq_nat (Suc j) (reach_count q a symbols d target states (Suc j)))
                  True)
                (stable_next q a symbols d target j st);
            Inr grown ↦
              Inr
                (stable q a symbols d target (Suc j))
                (Equal
                  Bool
                  (leq_nat (Suc j) (reach_count q a symbols d target states (Suc j)))
                  True)
                grown
          }
      }
  }

fn growth
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (states : List q)
      (cover : (x : q) → list_elem q x states)
      (k : Nat)
      (ge : Equal Bool (leq_nat k (reach_count q a symbols d target states k)) True)
    : Or
        (stable q a symbols d target k)
        (Equal Bool (leq_nat (Suc k) (reach_count q a symbols d target states (Suc k))) True) =
  growth_at
    q
    a
    symbols
    d
    target
    states
    cover
    k
    ge
    (leq_nat
      (reach_count q a symbols d target states (Suc k))
      (reach_count q a symbols d target states k))
    Refl

fn growth_at
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (states : List q)
      (cover : (x : q) → list_elem q x states)
      (k : Nat)
      (ge : Equal Bool (leq_nat k (reach_count q a symbols d target states k)) True)
      (settled : Bool)
    : Equal Bool
        (leq_nat
          (reach_count q a symbols d target states (Suc k))
          (reach_count q a symbols d target states k))
        settled
      → Or
        (stable q a symbols d target k)
        (Equal Bool (leq_nat (Suc k) (reach_count q a symbols d target states (Suc k))) True) =
  match settled {
    True ↦
      λback.
        Inl
          (stable q a symbols d target k)
          (Equal Bool (leq_nat (Suc k) (reach_count q a symbols d target states (Suc k))) True)
          (λs.
            count_back
              q
              (reach q a symbols d target k)
              (reach q a symbols d target (Suc k))
              (reach_mono q a symbols d target k)
              states
              back
              s
              (cover s));
    False ↦
      λgrew.
        Inr
          (stable q a symbols d target k)
          (Equal Bool (leq_nat (Suc k) (reach_count q a symbols d target states (Suc k))) True)
          ((proof trans for leq_nat)
            (Suc k)
            (Suc (reach_count q a symbols d target states k))
            (reach_count q a symbols d target states (Suc k))
            ge
            (not_leq_gives_lt
              (reach_count q a symbols d target states (Suc k))
              (reach_count q a symbols d target states k)
              grew))
  }

fn reach_count
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (states : List q)
      (k : Nat)
    : Nat =
  count_true q (reach q a symbols d target k) states

theorem stable_closed
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (n : Nat)
      (st : stable q a symbols d target n)
      (m : Nat)
    : (s : q)
      → Equal Bool (reach q a symbols d target m s) True
      → Equal Bool (reach q a symbols d target n s) True =
  match m {
    Zero ↦ reach_zero_up q a symbols d target n;
    Suc j ↦
      λs.
        λh.
          st
            s
            (or_mono
              (target s)
              (target s)
              (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols)
              (any_of a (λx. reach q a symbols d target n (step q a d s x)) symbols)
              (λe. e)
              (any_of_mono
                a
                (λx. reach q a symbols d target j (step q a d s x))
                (λx. reach q a symbols d target n (step q a d s x))
                (λx. stable_closed q a symbols d target n st j (step q a d s x))
                symbols)
              h)
  }

theorem reach_zero_up
      (q : Type) (a : Type) (symbols : List a) (d : Dfa q a) (target : q → Bool) (n : Nat)
    : (s : q)
      → Equal Bool (reach q a symbols d target Zero s) True
      → Equal Bool (reach q a symbols d target n s) True =
  match n {
    Zero ↦ λs. λh. h;
    Suc m ↦
      λs. λh. reach_mono q a symbols d target m s (reach_zero_up q a symbols d target m s h)
  }

theorem stable_next
      (q : Type) (a : Type) (symbols : List a) (d : Dfa q a) (target : q → Bool) (k : Nat)
    : stable q a symbols d target k → stable q a symbols d target (Suc k) =
  λst.
    λs.
      or_mono
        (target s)
        (target s)
        (any_of a (λx. reach q a symbols d target (Suc k) (step q a d s x)) symbols)
        (any_of a (λx. reach q a symbols d target k (step q a d s x)) symbols)
        (λh. h)
        (any_of_mono
          a
          (λx. reach q a symbols d target (Suc k) (step q a d s x))
          (λx. reach q a symbols d target k (step q a d s x))
          (λx. st (step q a d s x))
          symbols)

fn stable
      (q : Type) (a : Type) (symbols : List a) (d : Dfa q a) (target : q → Bool) (k : Nat)
    : Omega =
  (s : q)
    → Equal Bool
    (reach q a symbols d target (Suc k) s)
    True → Equal Bool
    (reach q a symbols d target k s)
    True

theorem reach_walk
      (q : Type)
      (a : Type)
      (symbols : List a)
      (cover : (x : a) → list_elem a x symbols)
      (d : Dfa q a)
      (target : q → Bool)
      (w : List a)
    : (s : q)
      → Equal Bool (target (run q a d s w)) True
      → Equal Bool (reach q a symbols d target (length a w) s) True =
  match w {
    Nil ↦ λs. λh. h;
    Cons x rest ↦
      λs.
        λh.
          or_right
            (target s)
            (any_of a (λy. reach q a symbols d target (length a rest) (step q a d s y)) symbols)
            (any_of_elem
              a
              (λy. reach q a symbols d target (length a rest) (step q a d s y))
              x
              symbols
              (cover x)
              (reach_walk q a symbols cover d target rest (step q a d s x) h))
  }

theorem reach_mono
      (q : Type) (a : Type) (symbols : List a) (d : Dfa q a) (target : q → Bool) (k : Nat)
    : (s : q)
      → Equal Bool (reach q a symbols d target k s) True
      → Equal Bool (reach q a symbols d target (Suc k) s) True =
  match k {
    Zero ↦
      λs.
        λh.
          or_left
            (target s)
            (any_of a (λx. reach q a symbols d target Zero (step q a d s x)) symbols)
            h;
    Suc j ↦
      λs.
        or_mono
          (target s)
          (target s)
          (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols)
          (any_of a (λx. reach q a symbols d target (Suc j) (step q a d s x)) symbols)
          (λh. h)
          (any_of_mono
            a
            (λx. reach q a symbols d target j (step q a d s x))
            (λx. reach q a symbols d target (Suc j) (step q a d s x))
            (λx. reach_mono q a symbols d target j (step q a d s x))
            symbols)
  }

fn reach
      (q : Type)
      (a : Type)
      (symbols : List a)
      (d : Dfa q a)
      (target : q → Bool)
      (k : Nat)
      (s : q)
    : Bool =
  match k {
    Zero ↦ target s;
    Suc j ↦
      bool_or (target s) (any_of a (λx. reach q a symbols d target j (step q a d s x)) symbols)
  }

theorem count_back
      (q : Type)
      (f : q → Bool)
      (g : q → Bool)
      (mono : (x : q) → Equal Bool (f x) True → Equal Bool (g x) True)
      (xs : List q)
    : Equal Bool (leq_nat (count_true q g xs) (count_true q f xs)) True
      → (x : q)
      → list_elem q x xs
      → Equal Bool (g x) True
      → Equal Bool (f x) True =
  match xs {
    Nil ↦ λback. λx. λmember. λh. absurd member;
    Cons y rest ↦
      λback.
        λx.
          λmember.
            λh.
              elim_trunc
                (Equal Bool (f x) True)
                (λcase.
                  match case {
                    Inl same ↦
                      trans
                        Bool
                        (f x)
                        (f y)
                        True
                        (cong q Bool x y f same)
                        (count_head_back
                          (f y)
                          (g y)
                          (count_true q f rest)
                          (count_true q g rest)
                          (mono y)
                          (count_mono q f g mono rest)
                          back
                          (trans
                            Bool
                            (g y)
                            (g x)
                            True
                            (sym Bool (g x) (g y) (cong q Bool x y g same))
                            h));
                    Inr later ↦
                      count_back
                        q
                        f
                        g
                        mono
                        rest
                        (count_tail_back
                          (f y)
                          (g y)
                          (count_true q f rest)
                          (count_true q g rest)
                          (mono y)
                          (count_mono q f g mono rest)
                          back)
                        x
                        later
                        h
                  })
                member
  }

theorem count_tail_back
      (b : Bool) (c : Bool) (cf : Nat) (cg : Nat)
    : (Equal Bool b True → Equal Bool c True)
      → Equal Bool (leq_nat cf cg) True
      → Equal Bool (leq_nat (bool_count c cg) (bool_count b cf)) True
      → Equal Bool (leq_nat cg cf) True =
  match b {
    True ↦
      match c {
        True ↦ λm. λle. λback. back;
        False ↦ λm. λle. λback. absurd (m Proved)
      };
    False ↦
      match c {
        True ↦
          λm.
            λle.
              λback.
                leq_suc_self_absurd
                  cg
                  (Equal Bool (leq_nat cg cf) True)
                  ((proof trans for leq_nat) (Suc cg) cf cg back le);
        False ↦ λm. λle. λback. back
      }
  }

theorem count_head_back
      (b : Bool) (c : Bool) (cf : Nat) (cg : Nat)
    : (Equal Bool b True → Equal Bool c True)
      → Equal Bool (leq_nat cf cg) True
      → Equal Bool (leq_nat (bool_count c cg) (bool_count b cf)) True
      → Equal Bool c True
      → Equal Bool b True =
  match b {
    True ↦ λm. λle. λback. λct. Proved;
    False ↦
      match c {
        True ↦
          λm.
            λle.
              λback.
                λct.
                  leq_suc_self_absurd
                    cg
                    (Equal Bool False True)
                    ((proof trans for leq_nat) (Suc cg) cf cg back le);
        False ↦ λm. λle. λback. λct. absurd ct
      }
  }

theorem count_mono
      (q : Type)
      (f : q → Bool)
      (g : q → Bool)
      (mono : (x : q) → Equal Bool (f x) True → Equal Bool (g x) True)
      (xs : List q)
    : Equal Bool (leq_nat (count_true q f xs) (count_true q g xs)) True =
  match xs {
    Nil ↦ Proved;
    Cons y rest ↦
      count_mono_step
        (f y)
        (g y)
        (count_true q f rest)
        (count_true q g rest)
        (mono y)
        (count_mono q f g mono rest)
  }

theorem count_mono_step
      (b : Bool) (c : Bool) (cf : Nat) (cg : Nat)
    : (Equal Bool b True → Equal Bool c True)
      → Equal Bool (leq_nat cf cg) True
      → Equal Bool (leq_nat (bool_count b cf) (bool_count c cg)) True =
  match b {
    True ↦
      match c {
        True ↦ λm. λh. h;
        False ↦ λm. λh. absurd (m Proved)
      };
    False ↦
      match c {
        True ↦ λm. λh. leq_nat_weaken_right cf cg h;
        False ↦ λm. λh. h
      }
  }

theorem count_bound
      (q : Type) (f : q → Bool) (xs : List q)
    : Equal Bool (leq_nat (count_true q f xs) (length q xs)) True =
  match xs {
    Nil ↦ Proved;
    Cons y rest ↦
      count_bound_step (f y) (count_true q f rest) (length q rest) (count_bound q f rest)
  }

theorem count_bound_step
      (b : Bool) (c : Nat) (n : Nat)
    : Equal Bool (leq_nat c n) True → Equal Bool (leq_nat (bool_count b c) (Suc n)) True =
  match b {
    True ↦ λh. h;
    False ↦ λh. leq_nat_weaken_right c n h
  }

fn count_true (q : Type) (f : q → Bool) (xs : List q) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons y rest ↦ bool_count (f y) (count_true q f rest)
  }

fn bool_count (b : Bool) (n : Nat) : Nat =
  match b {
    True ↦ Suc n;
    False ↦ n
  }

theorem any_of_elem
      (a : Type) (f : a → Bool) (x : a) (xs : List a)
    : list_elem a x xs → Equal Bool (f x) True → Equal Bool (any_of a f xs) True =
  match xs {
    Nil ↦ λmember. λh. absurd member;
    Cons y rest ↦
      λmember.
        λh.
          elim_trunc
            (Equal Bool (any_of a f (Cons a y rest)) True)
            (λcase.
              match case {
                Inl same ↦
                  or_left
                    (f y)
                    (any_of a f rest)
                    (trans
                      Bool
                      (f y)
                      (f x)
                      True
                      (sym Bool (f x) (f y) (cong a Bool x y f same))
                      h);
                Inr later ↦ or_right (f y) (any_of a f rest) (any_of_elem a f x rest later h)
              })
            member
  }

theorem any_of_ext
      (a : Type)
      (f : a → Bool)
      (g : a → Bool)
      (each : (x : a) → Equal Bool (f x) (g x))
      (xs : List a)
    : Equal Bool (any_of a f xs) (any_of a g xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦
      trans
        Bool
        (bool_or (f x) (any_of a f rest))
        (bool_or (g x) (any_of a f rest))
        (bool_or (g x) (any_of a g rest))
        (cong Bool Bool (f x) (g x) (λb. bool_or b (any_of a f rest)) (each x))
        (cong
          Bool
          Bool
          (any_of a f rest)
          (any_of a g rest)
          (λc. bool_or (g x) c)
          (any_of_ext a f g each rest))
  }

theorem any_of_mono
      (a : Type)
      (f : a → Bool)
      (g : a → Bool)
      (each : (x : a) → Equal Bool (f x) True → Equal Bool (g x) True)
      (xs : List a)
    : Equal Bool (any_of a f xs) True → Equal Bool (any_of a g xs) True =
  match xs {
    Nil ↦ λh. absurd h;
    Cons x rest ↦
      or_mono
        (f x)
        (g x)
        (any_of a f rest)
        (any_of a g rest)
        (each x)
        (any_of_mono a f g each rest)
  }

fn any_of (a : Type) (f : a → Bool) (xs : List a) : Bool =
  match xs {
    Nil ↦ False;
    Cons x rest ↦ bool_or (f x) (any_of a f rest)
  }

theorem or_mono
      (b : Bool) (b2 : Bool) (c : Bool) (c2 : Bool)
    : (Equal Bool b True → Equal Bool b2 True)
      → (Equal Bool c True → Equal Bool c2 True)
      → Equal Bool (bool_or b c) True
      → Equal Bool (bool_or b2 c2) True =
  match b {
    True ↦ λmb. λmc. λh. or_left b2 c2 (mb Proved);
    False ↦ λmb. λmc. λh. or_right b2 c2 (mc h)
  }

theorem not_leq_gives_lt
      (m : Nat)
    : (n : Nat) → Equal Bool (leq_nat m n) False → Equal Bool (leq_nat (Suc n) m) True =
  match m {
    Zero ↦ λn. λh. absurd h;
    Suc m2 ↦
      λn.
        match n {
          Zero ↦ λh. Proved;
          Suc n2 ↦ λh. not_leq_gives_lt m2 n2 h
        }
  }

theorem leq_suc_self_absurd
      (n : Nat) (goal : Omega)
    : Equal Bool (leq_nat (Suc n) n) True → goal =
  match n {
    Zero ↦ λh. absurd h;
    Suc m ↦ λh. leq_suc_self_absurd m goal h
  }
```

## 5. Design notes

The algorithm uses fuel bounded by the length of the supplied state list,
not by the length of a candidate word. Completeness is checked for every
witness word by a monotonicity and finite-counting argument. Neither shortest
witnesses nor runtime and space bounds are promised; certificates may contain
duplicates. Arbitrary-state `Dfa` values without finite certificates remain
valid but cannot use this decision procedure.

## 6. References

- [Graph reachability](https://en.wikipedia.org/wiki/Reachability) —
  Wikipedia; orientation to the question of reaching a target in a state
  transition graph.
- *Introduction to Automata Theory, Languages, and Computation* — Hopcroft,
  Motwani, and Ullman; context for finite-state language emptiness.

## 7. Trust & derivation

The contract is
[formal languages §2](../../../../spec/50-stdlib/61-formal-languages.md).
The public operations are `is_empty`, `accepted_word`, `reachable`, and
`find_word`; the four public checked laws are `is_empty_rejects`,
`accepted_word_accepts`, `find_word_complete`, and `find_word_sound`.

| Reader task | Section |
|---|---|
| Decide emptiness or request a word | [Definition](#2-definition), [Using it](#3-using-it) |
| Review soundness and completeness | [Laws & proofs](#4-laws--proofs) |
| Review limits and trust | [Design notes](#5-design-notes), [Trust & derivation](#7-trust--derivation) |

The derivation uses checked inductive `List` and `Nat`, ordinary Boolean
operations, the Dfa operations, `Finite` certificates, and generic transport.
The Nat order transitivity proof belongs to imported `leq_nat::trans` and
right weakening to imported `leq_nat_weaken_right`, not a local stand-in.
Private `count_true` could use `length` after `filter`, but direct recursion
exposes the `bool_count` step that the monotonicity and bound proofs induct
on; it is not a second public counting operation. No local Axiom, primitive,
or `trusted_base()` entry is added. Imported
providers' inherited trust remains visible to their own accounting.
