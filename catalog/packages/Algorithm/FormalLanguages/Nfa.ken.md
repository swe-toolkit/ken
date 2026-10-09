# `Nfa` — checked nondeterministic finite automata

An NFA tests edges with a Boolean relation and accepts a word when an
initial state has a path ending in a final state. A finite certificate for
states supports an equivalent deterministic Boolean view; a second
certificate for symbols makes language emptiness decidable.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

An NFA can have several initial states and several successors on a symbol.
Its path semantics remains meaningful without finite carriers. A supplied
`Finite q` lets `determinize` represent a subset as one Boolean bit per
listed position, even if the enumeration repeats a state. Only the finite
emptiness decision also needs `Finite a` to search over input symbols.

## 2. Definition

`Nfa` stores the edge relation, the initial-state test, and the final-state
test, in that order. `path_accepts` checks exactly one path state per input
symbol and checks finality at the endpoint. `NfaAcceptance` retains the
starting state and path as evidence; `nfa_accepts` truncates their existence
to a proposition.

The subset state's bit positions come from `elements q fq`. A successor bit
is set when some currently set source position has a forward edge to its
target; a final bit contributes only when its state satisfies `nfa_final`.

```ken
import Algorithm.FormalLanguages.Dfa (Dfa, MkDfa, run, accepts)

import Algorithm.FormalLanguages.Reachability (is_empty, is_empty_rejects)

import Data.Finite.Finite (Finite, elements, covers, pair_finite, unit_finite, bool_finite)

import Data.Collections.Derived (list_elem)

import Core.Logic.And (And, Both)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (sym)

import Core.Classes.LawfulClasses
  (bool_or, bool_and, or_left, or_right, or_cases, and_true, and_cases)

pub data Nfa q a = MkNfa (q → a → q → Bool) (q → Bool) (q → Bool)

export MkNfa

pub fn nfa_step (q : Type) (a : Type) (n : Nfa q a) (s : q) (x : a) (t : q) : Bool =
  match n {
    MkNfa transition initial accepting ↦ transition s x t
  }

pub fn nfa_initial (q : Type) (a : Type) (n : Nfa q a) (s : q) : Bool =
  match n {
    MkNfa transition initial accepting ↦ initial s
  }

pub fn nfa_final (q : Type) (a : Type) (n : Nfa q a) (s : q) : Bool =
  match n {
    MkNfa transition initial accepting ↦ accepting s
  }

pub fn path_accepts
      (q : Type) (a : Type) (n : Nfa q a) (s : q) (w : List a) (path : List q)
    : Omega =
  match w {
    Nil ↦
      match path {
        Nil ↦ Equal Bool (nfa_final q a n s) True;
        Cons t ts ↦ Bottom
      };
    Cons x rest ↦
      match path {
        Nil ↦ Bottom;
        Cons t ts ↦
          (‖ And (Equal Bool (nfa_step q a n s x t) True) (path_accepts q a n t rest ts) ‖)
      }
  }

pub data NfaAcceptance (q : Type) (a : Type) (n : Nfa q a) (w : List a) : Type where {
  Accepted :
    (s : q)
    → (path : List q)
    → Equal Bool (nfa_initial q a n s) True
    → path_accepts q a n s w path
    → NfaAcceptance q a n w
}

export Accepted

pub fn nfa_accepts (q : Type) (a : Type) (n : Nfa q a) (w : List a) : Omega =
  ‖ NfaAcceptance q a n w ‖

pub fn subset_state (q : Type) (fq : Finite q) : Type = mask q (elements q fq)

pub fn determinize
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a)
    : Dfa (subset_state q fq) a =
  MkDfa
    (subset_state q fq)
    a
    (subset_next q a fq n)
    (mask_build q (nfa_initial q a n) (elements q fq))
    (λm. mask_any q (nfa_final q a n) (elements q fq) m)

pub fn subset_finite (q : Type) (fq : Finite q) : Finite (subset_state q fq) =
  mask_finite q (elements q fq)

pub fn nfa_is_empty (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (n : Nfa q a) : Bool =
  is_empty (subset_state q fq) a (subset_finite q fq) fa (determinize q a fq n)

fn subset_next
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (m : subset_state q fq) (x : a)
    : subset_state q fq =
  mask_build q (λt. mask_any q (λs. nfa_step q a n s x t) (elements q fq) m) (elements q fq)
```

## 3. Using it

This one-state machine accepts the empty word. Its explicit certificate
allows the same machine to be determinized without deciding equality on
states or symbols.

```ken example
import Data.Finite.Finite (unit_finite)

const initial_and_final : Nfa Unit Bool =
  MkNfa Unit Bool (λs. λx. λt. False) (λs. True) (λs. True)

const same_language : Dfa (subset_state Unit unit_finite) Bool =
  determinize Unit Bool unit_finite initial_and_final
```

## 4. Laws & proofs

`determinize_sound` traces each set final-state bit backwards through the
word, constructing an initial state and an accepting path. Conversely,
`determinize_complete` follows any accepting path forwards, using `covers`
to maintain its active bit. `nfa_is_empty_rejects` combines the second
law with finite DFA emptiness: an accepting NFA path would contradict a
`True` emptiness decision.

Private mask operations and induction helpers below carry the same proofs;
`any_intro` and `any_elim` reuse public Boolean truth lemmas from
`Core.Classes.LawfulClasses`. None is another public language-equality or
path representation.

```ken
pub theorem determinize_sound
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a)
    : Equal Bool (accepts (subset_state q fq) a (determinize q a fq n) w) True
      → nfa_accepts q a n w =
  λh.
    elim_trunc
      (nfa_accepts q a n w)
      (λr.
        match r {
          MkReached s path held ok ↦
            trunc_intro
              (Accepted
                q
                a
                n
                w
                s
                path
                (build_bit q (nfa_initial q a n) (elements q fq) s held)
                ok)
        })
      (run_sound q a fq n w (mask_build q (nfa_initial q a n) (elements q fq)) h)

pub theorem determinize_complete
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a)
    : nfa_accepts q a n w
      → Equal Bool (accepts (subset_state q fq) a (determinize q a fq n) w) True =
  λacc.
    elim_trunc
      (Equal Bool (accepts (subset_state q fq) a (determinize q a fq n) w) True)
      (λc.
        match c {
          Accepted s path init ok ↦
            run_complete
              q
              a
              fq
              n
              w
              s
              path
              (mask_build q (nfa_initial q a n) (elements q fq))
              (build_holds q (nfa_initial q a n) (elements q fq) s (covers q fq s) init)
              ok
        })
      acc

pub theorem nfa_is_empty_rejects
      (q : Type) (a : Type) (fq : Finite q) (fa : Finite a) (n : Nfa q a) (w : List a)
    : Equal Bool (nfa_is_empty q a fq fa n) True → nfa_accepts q a n w → Bottom =
  λe.
    λacc.
      absurd_bool
        (accepts (subset_state q fq) a (determinize q a fq n) w)
        (is_empty_rejects
          (subset_state q fq)
          a
          (subset_finite q fq)
          fa
          (determinize q a fq n)
          w
          e)
        (determinize_complete q a fq n w acc)

theorem absurd_bool (b : Bool) : Equal Bool b False → Equal Bool b True → Bottom =
  match b {
    True ↦ λf. λt. absurd f;
    False ↦ λf. λt. absurd t
  }

data Reached
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (m : subset_state q fq) (w : List a)
    : Type
    where {
  MkReached :
    (s : q)
    → (path : List q)
    → holds q s (elements q fq) m
    → path_accepts q a n s w path
    → Reached q a fq n m w
}

theorem run_sound
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a)
    : (m : subset_state q fq)
      → Equal Bool
        (mask_any
          q
          (nfa_final q a n)
          (elements q fq)
          (run (subset_state q fq) a (determinize q a fq n) m w))
        True
      → ‖ Reached q a fq n m w ‖ =
  match w {
    Nil ↦
      λm.
        λh.
          any_elim
            q
            (nfa_final q a n)
            (elements q fq)
            m
            h
            (‖ Reached q a fq n m (Nil a) ‖)
            (sound_end q a fq n m);
    Cons x rest ↦
      λm.
        λh.
          elim_trunc
            (‖ Reached q a fq n m (Cons a x rest) ‖)
            (sound_step q a fq n x rest m)
            (run_sound q a fq n rest (subset_next q a fq n m x) h)
  }

theorem sound_end
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (n : Nfa q a)
      (m : subset_state q fq)
      (s : q)
      (held : holds q s (elements q fq) m)
      (fin : Equal Bool (nfa_final q a n s) True)
    : ‖ Reached q a fq n m (Nil a) ‖ =
  trunc_intro (MkReached q a fq n m (Nil a) s (Nil q) held fin)

theorem sound_step
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (n : Nfa q a)
      (x : a)
      (rest : List a)
      (m : subset_state q fq)
      (r : Reached q a fq n (subset_next q a fq n m x) rest)
    : ‖ Reached q a fq n m (Cons a x rest) ‖ =
  match r {
    MkReached t path held ok ↦
      any_elim
        q
        (λs. nfa_step q a n s x t)
        (elements q fq)
        m
        (build_bit
          q
          (λt2. mask_any q (λs. nfa_step q a n s x t2) (elements q fq) m)
          (elements q fq)
          t
          held)
        (‖ Reached q a fq n m (Cons a x rest) ‖)
        (sound_edge q a fq n x rest m t path ok)
  }

theorem sound_edge
      (q : Type)
      (a : Type)
      (fq : Finite q)
      (n : Nfa q a)
      (x : a)
      (rest : List a)
      (m : subset_state q fq)
      (t : q)
      (path : List q)
      (ok : path_accepts q a n t rest path)
      (s : q)
      (held : holds q s (elements q fq) m)
      (edge : Equal Bool (nfa_step q a n s x t) True)
    : ‖ Reached q a fq n m (Cons a x rest) ‖ =
  trunc_intro
    (MkReached
      q
      a
      fq
      n
      m
      (Cons a x rest)
      s
      (Cons q t path)
      held
      (trunc_intro
        (Both
          (Equal Bool (nfa_step q a n s x t) True)
          (path_accepts q a n t rest path)
          edge
          ok)))

theorem run_complete
      (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a)
    : (s : q)
      → (path : List q)
      → (m : subset_state q fq)
      → holds q s (elements q fq) m
      → path_accepts q a n s w path
      → Equal Bool
        (mask_any
          q
          (nfa_final q a n)
          (elements q fq)
          (run (subset_state q fq) a (determinize q a fq n) m w))
        True =
  match w {
    Nil ↦
      λs.
        λpath.
          match path {
            Nil ↦ λm. λheld. λfin. any_intro q (nfa_final q a n) (elements q fq) m s held fin;
            Cons t ts ↦ λm. λheld. λok. absurd ok
          };
    Cons x rest ↦
      λs.
        λpath.
          match path {
            Nil ↦ λm. λheld. λok. absurd ok;
            Cons t ts ↦
              λm.
                λheld.
                  λok.
                    elim_trunc
                      (Equal
                        Bool
                        (mask_any
                          q
                          (nfa_final q a n)
                          (elements q fq)
                          (run (subset_state q fq) a (determinize q a fq n) m (Cons a x rest)))
                        True)
                      (λboth.
                        match both {
                          Both dt okt ↦
                            let successor_bit : q → Bool =
                              λt2. mask_any q (λs2. nfa_step q a n s2 x t2) (elements q fq) m
                            in
                              run_complete
                                q
                                a
                                fq
                                n
                                rest
                                t
                                ts
                                (mask_build q successor_bit (elements q fq))
                                (build_holds
                                  q
                                  successor_bit
                                  (elements q fq)
                                  t
                                  (covers q fq t)
                                  (any_intro
                                    q
                                    (λs2. nfa_step q a n s2 x t)
                                    (elements q fq)
                                    m
                                    s
                                    held
                                    dt))
                                okt
                        })
                      ok
          }
  }

fn mask (q : Type) (xs : List q) : Type =
  match xs {
    Nil ↦ Unit;
    Cons h t ↦ Pair Bool (mask q t)
  }

fn mask_any (q : Type) (p : q → Bool) (xs : List q) : mask q xs → Bool =
  match xs {
    Nil ↦ λm. False;
    Cons h t ↦
      λm.
        bool_or
          (bool_and (pair_fst Bool (mask q t) m) (p h))
          (mask_any q p t (pair_snd Bool (mask q t) m))
  }

fn mask_build (q : Type) (f : q → Bool) (xs : List q) : mask q xs =
  match xs {
    Nil ↦ MkUnit;
    Cons h t ↦ mk_pair Bool (mask q t) (f h) (mask_build q f t)
  }

fn holds (q : Type) (s : q) (xs : List q) : mask q xs → Omega =
  match xs {
    Nil ↦ λm. Bottom;
    Cons h t ↦
      λm.
        ‖ Or
          (‖ And (Equal q s h) (Equal Bool (pair_fst Bool (mask q t) m) True) ‖)
          (holds q s t (pair_snd Bool (mask q t) m)) ‖
  }

fn mask_finite (q : Type) (xs : List q) : Finite (mask q xs) =
  match xs {
    Nil ↦ unit_finite;
    Cons h t ↦ pair_finite Bool (mask q t) bool_finite (mask_finite q t)
  }

theorem any_intro
      (q : Type) (p : q → Bool) (xs : List q)
    : (m : mask q xs)
      → (s : q)
      → holds q s xs m
      → Equal Bool (p s) True
      → Equal Bool (mask_any q p xs m) True =
  match xs {
    Nil ↦ λm. λs. λheld. λps. absurd held;
    Cons h t ↦
      λm.
        λs.
          λheld.
            λps.
              elim_trunc
                (Equal Bool (mask_any q p (Cons q h t) m) True)
                (λcase.
                  match case {
                    Inl here ↦
                      elim_trunc
                        (Equal Bool (mask_any q p (Cons q h t) m) True)
                        (λboth.
                          match both {
                            Both same bit ↦
                              or_left
                                (bool_and (pair_fst Bool (mask q t) m) (p h))
                                (mask_any q p t (pair_snd Bool (mask q t) m))
                                (and_true
                                  (pair_fst Bool (mask q t) m)
                                  (p h)
                                  bit
                                  (J (λy _. Equal Bool (p y) True) ps same))
                          })
                        here;
                    Inr later ↦
                      or_right
                        (bool_and (pair_fst Bool (mask q t) m) (p h))
                        (mask_any q p t (pair_snd Bool (mask q t) m))
                        (any_intro q p t (pair_snd Bool (mask q t) m) s later ps)
                  })
                held
  }

theorem any_elim
      (q : Type) (p : q → Bool) (xs : List q)
    : (m : mask q xs)
      → Equal Bool (mask_any q p xs m) True
      → (g : Omega)
      → ((s : q) → holds q s xs m → Equal Bool (p s) True → g)
      → g =
  match xs {
    Nil ↦ λm. λh. λg. λk. absurd h;
    Cons h t ↦
      λm.
        λe.
          λg.
            λk.
              or_cases
                (bool_and (pair_fst Bool (mask q t) m) (p h))
                (mask_any q p t (pair_snd Bool (mask q t) m))
                g
                e
                (λfirst.
                  and_cases
                    (pair_fst Bool (mask q t) m)
                    (p h)
                    g
                    first
                    (λbit.
                      λph.
                        k
                          h
                          (trunc_intro
                            (Inl
                              (‖ And
                                (Equal q h h)
                                (Equal Bool (pair_fst Bool (mask q t) m) True) ‖)
                              (holds q h t (pair_snd Bool (mask q t) m))
                              (trunc_intro
                                (Both
                                  (Equal q h h)
                                  (Equal Bool (pair_fst Bool (mask q t) m) True)
                                  Refl
                                  bit))))
                          ph))
                (λlater.
                  any_elim
                    q
                    p
                    t
                    (pair_snd Bool (mask q t) m)
                    later
                    g
                    (λs.
                      λheld.
                        λps.
                          k
                            s
                            (trunc_intro
                              (Inr
                                (‖ And
                                  (Equal q s h)
                                  (Equal Bool (pair_fst Bool (mask q t) m) True) ‖)
                                (holds q s t (pair_snd Bool (mask q t) m))
                                held))
                            ps))
  }

theorem build_holds
      (q : Type) (f : q → Bool) (xs : List q)
    : (s : q) → list_elem q s xs → Equal Bool (f s) True → holds q s xs (mask_build q f xs) =
  match xs {
    Nil ↦ λs. λmember. λfs. absurd member;
    Cons h t ↦
      λs.
        λmember.
          λfs.
            elim_trunc
              (holds q s (Cons q h t) (mask_build q f (Cons q h t)))
              (λcase.
                match case {
                  Inl same ↦
                    trunc_intro
                      (Inl
                        (‖ And (Equal q s h) (Equal Bool (f h) True) ‖)
                        (holds q s t (mask_build q f t))
                        (trunc_intro
                          (Both
                            (Equal q s h)
                            (Equal Bool (f h) True)
                            same
                            (J (λy _. Equal Bool (f y) True) fs same))));
                  Inr later ↦
                    trunc_intro
                      (Inr
                        (‖ And (Equal q s h) (Equal Bool (f h) True) ‖)
                        (holds q s t (mask_build q f t))
                        (build_holds q f t s later fs))
                })
              member
  }

theorem build_bit
      (q : Type) (f : q → Bool) (xs : List q)
    : (s : q) → holds q s xs (mask_build q f xs) → Equal Bool (f s) True =
  match xs {
    Nil ↦ λs. λheld. absurd held;
    Cons h t ↦
      λs.
        λheld.
          elim_trunc
            (Equal Bool (f s) True)
            (λcase.
              match case {
                Inl here ↦
                  elim_trunc
                    (Equal Bool (f s) True)
                    (λboth.
                      match both {
                        Both same bit ↦ J (λy _. Equal Bool (f y) True) bit (sym q s h same)
                      })
                    here;
                Inr later ↦ build_bit q f t s later
              })
            held
  }
```

## 5. Design notes

The `mask` carrier is `Unit` for an empty enumeration and
`Pair Bool (mask q rest)` for a nonempty one. Its representation and
witness lemmas stay private; `subset_state` and `subset_finite` are the
public views. The latter combines `unit_finite`, `bool_finite`, and
`pair_finite`, with no class inference or state equality. The construction
makes no time, space, or shortest-path claim, and has no epsilon edges.

## 6. References

- [Nondeterministic finite automaton](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton)
  — Wikipedia; orientation to accepting paths and the subset construction.
- *Introduction to Automata Theory, Languages, and Computation* — Hopcroft,
  Motwani, and Ullman; background on language equivalence and determinization.
- [Formal languages §3](../../../../spec/50-stdlib/61-formal-languages.md)
  — the NFA carrier, path, proof, and emptiness contract.

## 7. Trust & derivation

The public surface comprises `Nfa`, `MkNfa`, its three projections,
`path_accepts`, `NfaAcceptance`, `Accepted`, `nfa_accepts`, `subset_state`,
`determinize`, `subset_finite`, `nfa_is_empty`, `determinize_sound`,
`determinize_complete`, and `nfa_is_empty_rejects`.

| Reader task | Section |
|---|---|
| Model an accepting path | [Definition](#2-definition), [Using it](#3-using-it) |
| Decide language emptiness | [Definition](#2-definition), [Laws & proofs](#4-laws--proofs) |
| Review certificate and trust | [Laws & proofs](#4-laws--proofs), [Design notes](#5-design-notes) |

The definitions use ordinary checked lists, Boolean tests, inductive data,
truncation, and the `Dfa`, `Reachability`, `Finite`, `Derived`, `And`, `Or`,
and transport packages. The accepted-path witness and both language
implications have kernel-checked proof terms. This package adds no local
Axiom, postulate, primitive, foreign declaration, or `trusted_base()` entry;
assumptions inherited through its providers remain inherited. Its public
behavior and trust boundary are exercised by the NFA conformance seed
[seed-nfa](../../../../conformance/stdlib/formal-languages/seed-nfa.md).
