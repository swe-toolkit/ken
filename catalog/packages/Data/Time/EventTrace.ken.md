# Finite event traces

Finite events with natural-number times support half-open windows and a
left-biased two-list merge with checked ordering, count, membership and
window/merge distribution laws. Merge membership holds even for unordered
inputs.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A finite trace is an ordinary list of events, so callers can carry ordered
or unordered input without wrapping it in another type. Each event records
a given `Nat` timestamp and an arbitrary payload. Windowing filters the
existing trace; checked membership laws characterize which events remain.
Merging compares current heads and preserves each input's relative order.
Neither operation reads a clock or sorts unordered input.

## 2. Definition

The public carrier, operations and the first five generic laws lead the
checked source, followed by their private recursive merge helper and proof
lemmas. The window/merge distribution law and the three merge membership
laws each lead their own private lemmas later in the source.
Windowing includes the lower bound but excludes the upper; a merge tie favors
the left input without examining payloads. Membership uses Ω-valued `list_elem`,
so neither a payload equality decision nor an ordering premise is needed.

```ken
import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)

import Core.Classes.LawfulClasses (IsTrue, and_true, bool_and, bool_or, leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Core.Logic.Or (Or, Inl, Inr)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived (is_sorted)

import Data.Collections.Derived
  (filter,
    count,
    list_elem,
    list_elem_later,
    list_elem_filter_sound,
    list_elem_filter_complete,
    starts_after,
    list_sorted_cons,
    list_sorted_tail,
    list_sorted_head,
    filter_sorted,
    filter_starts_after,
    count_left_first,
    count_right_first)

pub data Event (p : Type) : Type where {
  MkEvent : (time : Nat) → (payload : p) → Event p
}

export MkEvent

pub fn event_time (p : Type) (e : Event p) : Nat =
  match e {
    MkEvent t x ↦ t
  }

pub fn event_payload (p : Type) (e : Event p) : p =
  match e {
    MkEvent t x ↦ x
  }

pub fn event_before (p : Type) (e : Event p) (f : Event p) : Bool =
  leq_nat (event_time p e) (event_time p f)

pub fn time_ordered (p : Type) (tr : List (Event p)) : Prop =
  is_sorted (Event p) (event_before p) tr

pub fn event_in_window (p : Type) (lo : Nat) (hi : Nat) (e : Event p) : Bool =
  bool_and (leq_nat lo (event_time p e)) (lt_nat (event_time p e) hi)

pub fn trace_window (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) : List (Event p) =
  filter (Event p) (event_in_window p lo hi) tr

pub fn trace_merge (p : Type) (xs : List (Event p)) (ys : List (Event p)) : List (Event p) =
  match xs {
    Nil ↦ ys;
    Cons x xt ↦ trace_merge_into p x xt (trace_merge p xt) ys
  }

pub theorem trace_window_ordered
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr → time_ordered p (trace_window p lo hi tr) =
  filter_sorted (Event p) (event_before p) (event_before_trans p) (event_in_window p lo hi) tr

pub theorem trace_window_member_sound
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) (e : Event p)
    : list_elem (Event p) e (trace_window p lo hi tr)
      → And
        (list_elem (Event p) e tr)
        (And
          (Equal Bool (leq_nat lo (event_time p e)) True)
          (Equal Bool (lt_nat (event_time p e) hi) True)) =
  λmember.
    let
      filtered_evidence =
        list_elem_filter_sound (Event p) (event_in_window p lo hi) e tr member;
      window_bounds : Equal Bool (event_in_window p lo hi e) True =
        and_snd
          (list_elem (Event p) e tr)
          (Equal Bool (event_in_window p lo hi e) True)
          filtered_evidence
    in
      and_intro
        (list_elem (Event p) e tr)
        (And
          (Equal Bool (leq_nat lo (event_time p e)) True)
          (Equal Bool (lt_nat (event_time p e) hi) True))
        (and_fst
          (list_elem (Event p) e tr)
          (Equal Bool (event_in_window p lo hi e) True)
          filtered_evidence)
        (and_intro
          (Equal Bool (leq_nat lo (event_time p e)) True)
          (Equal Bool (lt_nat (event_time p e) hi) True)
          ((proof left for bool_and)
            (leq_nat lo (event_time p e))
            (lt_nat (event_time p e) hi)
            window_bounds)
          ((proof right for bool_and)
            (leq_nat lo (event_time p e))
            (lt_nat (event_time p e) hi)
            window_bounds))

pub theorem trace_window_member_complete
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) (e : Event p)
    : list_elem (Event p) e tr
      → Equal Bool (leq_nat lo (event_time p e)) True
      → Equal Bool (lt_nat (event_time p e) hi) True
      → list_elem (Event p) e (trace_window p lo hi tr) =
  λmember.
    λabove.
      λbelow.
        list_elem_filter_complete
          (Event p)
          (event_in_window p lo hi)
          e
          tr
          member
          (and_true (leq_nat lo (event_time p e)) (lt_nat (event_time p e) hi) above below)

pub theorem trace_merge_ordered
      (p : Type) (xs : List (Event p))
    : (ys : List (Event p))
      → time_ordered p xs
      → time_ordered p ys
      → time_ordered p (trace_merge p xs ys) =
  match xs {
    Nil ↦ λys. λsx. λsy. sy;
    Cons x xt ↦
      λys.
        λsx.
          λsy.
            trace_merge_into_sorted
              p
              x
              xt
              (trace_merge p xt)
              (λzs.
                λsz.
                  trace_merge_ordered
                    p
                    xt
                    zs
                    (list_sorted_tail (Event p) (event_before p) x xt sx)
                    sz)
              (λzs.
                λhz.
                  trace_merge_starts_after
                    p
                    x
                    xt
                    zs
                    (list_sorted_head (Event p) (event_before p) x xt sx)
                    hz)
              ys
              sx
              sy
  }

pub theorem trace_merge_count
      (p : Type) (eqf : Event p → Event p → Bool) (q : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → Equal Nat
        (count (Event p) eqf q (trace_merge p xs ys))
        (add (count (Event p) eqf q xs) (count (Event p) eqf q ys)) =
  match xs {
    Nil ↦
      λys.
        sym
          Nat
          (add Zero (count (Event p) eqf q ys))
          (count (Event p) eqf q ys)
          ((proof zero_l for add) (count (Event p) eqf q ys));
    Cons x xt ↦
      λys.
        trace_merge_into_count
          p
          eqf
          q
          x
          xt
          (trace_merge p xt)
          (λzs. trace_merge_count p eqf q xt zs)
          ys
  }

fn trace_merge_into
      (p : Type)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (ys : List (Event p))
    : List (Event p) =
  match ys {
    Nil ↦ Cons (Event p) x xt;
    Cons y yt ↦
      match event_before p x y {
        True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
        False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
      }
  }

theorem event_before_trans
      (p : Type) (e : Event p) (f : Event p) (g : Event p)
    : Equal Bool (event_before p e f) True
      → Equal Bool (event_before p f g) True
      → Equal Bool (event_before p e g) True =
  (proof trans for leq_nat) (event_time p e) (event_time p f) (event_time p g)

theorem event_before_flip
      (p : Type)
      (e : Event p)
      (f : Event p)
      (left_false : Equal Bool (event_before p e f) False)
    : Equal Bool (event_before p f e) True =
  trans
    Bool
    (event_before p f e)
    (bool_or (event_before p e f) (event_before p f e))
    True
    (sym
      Bool
      (bool_or (event_before p e f) (event_before p f e))
      (event_before p f e)
      (cong
        Bool
        Bool
        (event_before p e f)
        False
        (λdecision. bool_or decision (event_before p f e))
        left_false))
    ((proof total for leq_nat) (event_time p e) (event_time p f))

theorem trace_merge_into_starts_after_cons
      (p : Type)
      (h : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (y : Event p)
      (yt : List (Event p))
      (hx : Equal Bool (event_before p h x) True)
      (hy : Equal Bool (event_before p h y) True)
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → starts_after
        (Event p)
        (event_before p)
        h
        (trace_merge_into p x xt rest (Cons (Event p) y yt)) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            starts_after
              (Event p)
              (event_before p)
              h
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          hx
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            starts_after
              (Event p)
              (event_before p)
              h
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          hy
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_starts_after
      (p : Type)
      (h : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (ys : List (Event p))
    : Equal Bool (event_before p h x) True
      → starts_after (Event p) (event_before p) h ys
      → starts_after (Event p) (event_before p) h (trace_merge_into p x xt rest ys) =
  match ys {
    Nil ↦ λhx. λhys. hx;
    Cons y yt ↦
      λhx.
        λhys.
          trace_merge_into_starts_after_cons p h x xt rest y yt hx hys (event_before p x y) Refl
  }

theorem trace_merge_starts_after
      (p : Type) (h : Event p) (xs : List (Event p)) (ys : List (Event p))
    : starts_after (Event p) (event_before p) h xs
      → starts_after (Event p) (event_before p) h ys
      → starts_after (Event p) (event_before p) h (trace_merge p xs ys) =
  match xs {
    Nil ↦ λhxs. λhys. hys;
    Cons x xt ↦ λhxs. λhys. trace_merge_into_starts_after p h x xt (trace_merge p xt) ys hxs hys
  }

theorem trace_merge_into_sorted_cons
      (p : Type)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_sorted : (zs : List (Event p)) → time_ordered p zs → time_ordered p (rest zs))
      (rest_after : (zs : List (Event p))
        → starts_after
        (Event p)
        (event_before p)
        x
        zs
        → starts_after
        (Event p)
        (event_before p)
        x
        (rest zs))
      (y : Event p)
      (yt : List (Event p))
      (sorted_ys : time_ordered p (Cons (Event p) y yt))
      (ih : time_ordered p (trace_merge_into p x xt rest yt))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → time_ordered p (trace_merge_into p x xt rest (Cons (Event p) y yt)) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            is_sorted
              (Event p)
              (event_before p)
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (list_sorted_cons
            (Event p)
            (event_before p)
            x
            (rest (Cons (Event p) y yt))
            (rest_sorted (Cons (Event p) y yt) sorted_ys)
            (rest_after (Cons (Event p) y yt) comparison))
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            is_sorted
              (Event p)
              (event_before p)
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (list_sorted_cons
            (Event p)
            (event_before p)
            y
            (trace_merge_into p x xt rest yt)
            ih
            (trace_merge_into_starts_after
              p
              y
              x
              xt
              rest
              yt
              (event_before_flip p x y comparison)
              (list_sorted_head (Event p) (event_before p) y yt sorted_ys)))
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_sorted
      (p : Type)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_sorted : (zs : List (Event p)) → time_ordered p zs → time_ordered p (rest zs))
      (rest_after : (zs : List (Event p))
        → starts_after
        (Event p)
        (event_before p)
        x
        zs
        → starts_after
        (Event p)
        (event_before p)
        x
        (rest zs))
      (ys : List (Event p))
    : time_ordered p (Cons (Event p) x xt)
      → time_ordered p ys
      → time_ordered p (trace_merge_into p x xt rest ys) =
  match ys {
    Nil ↦ λsorted_xs. λsorted_ys. sorted_xs;
    Cons y yt ↦
      λsorted_xs.
        λsorted_ys.
          trace_merge_into_sorted_cons
            p
            x
            xt
            rest
            rest_sorted
            rest_after
            y
            yt
            sorted_ys
            (trace_merge_into_sorted
              p
              x
              xt
              rest
              rest_sorted
              rest_after
              yt
              sorted_xs
              (list_sorted_tail (Event p) (event_before p) y yt sorted_ys))
            (event_before p x y)
            Refl
  }

theorem trace_merge_into_count_cons
      (p : Type)
      (eqf : Event p → Event p → Bool)
      (q : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_count : (zs : List (Event p))
        → Equal
        Nat
        (count (Event p) eqf q (rest zs))
        (add (count (Event p) eqf q xt) (count (Event p) eqf q zs)))
      (y : Event p)
      (yt : List (Event p))
      (ih : Equal
        Nat
        (count (Event p) eqf q (trace_merge_into p x xt rest yt))
        (add (count (Event p) eqf q (Cons (Event p) x xt)) (count (Event p) eqf q yt)))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → Equal Nat
        (count (Event p) eqf q (trace_merge_into p x xt rest (Cons (Event p) y yt)))
        (add
          (count (Event p) eqf q (Cons (Event p) x xt))
          (count (Event p) eqf q (Cons (Event p) y yt))) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            Equal
              Nat
              (count
                (Event p)
                eqf
                q
                (match choice {
                  True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                  False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
                }))
              (add
                (count (Event p) eqf q (Cons (Event p) x xt))
                (count (Event p) eqf q (Cons (Event p) y yt))))
          (count_left_first
            (Event p)
            eqf
            q
            x
            xt
            (rest (Cons (Event p) y yt))
            (Cons (Event p) y yt)
            (rest_count (Cons (Event p) y yt)))
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            Equal
              Nat
              (count
                (Event p)
                eqf
                q
                (match choice {
                  True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                  False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
                }))
              (add
                (count (Event p) eqf q (Cons (Event p) x xt))
                (count (Event p) eqf q (Cons (Event p) y yt))))
          (count_right_first
            (Event p)
            eqf
            q
            (count (Event p) eqf q (Cons (Event p) x xt))
            y
            yt
            (trace_merge_into p x xt rest yt)
            ih)
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_count
      (p : Type)
      (eqf : Event p → Event p → Bool)
      (q : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_count : (zs : List (Event p))
        → Equal
        Nat
        (count (Event p) eqf q (rest zs))
        (add (count (Event p) eqf q xt) (count (Event p) eqf q zs)))
      (ys : List (Event p))
    : Equal Nat
        (count (Event p) eqf q (trace_merge_into p x xt rest ys))
        (add (count (Event p) eqf q (Cons (Event p) x xt)) (count (Event p) eqf q ys)) =
  match ys {
    Nil ↦ Refl;
    Cons y yt ↦
      trace_merge_into_count_cons
        p
        eqf
        q
        x
        xt
        rest
        rest_count
        y
        yt
        (trace_merge_into_count p eqf q x xt rest rest_count yt)
        (event_before p x y)
        Refl
  }

pub theorem trace_window_merge
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p))
    : (ys : List (Event p))
      → time_ordered p xs
      → time_ordered p ys
      → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge p xs ys))
        (trace_merge p (trace_window p lo hi xs) (trace_window p lo hi ys)) =
  match xs {
    Nil ↦ λys. λsx. λsy. Refl;
    Cons x xt ↦
      λys.
        λsx.
          λsy.
            trace_window_merge_into
              p
              lo
              hi
              x
              xt
              (λzs.
                λsz.
                  trace_window_merge
                    p
                    lo
                    hi
                    xt
                    zs
                    (list_sorted_tail (Event p) (event_before p) x xt sx)
                    sz)
              ys
              sx
              sy
  }

theorem trace_window_merge_into
      (p : Type)
      (lo : Nat)
      (hi : Nat)
      (x : Event p)
      (xt : List (Event p))
      (ih : (zs : List (Event p))
        → time_ordered
        p
        zs
        → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge p xt zs))
        (trace_merge p (trace_window p lo hi xt) (trace_window p lo hi zs)))
      (ys : List (Event p))
    : time_ordered p (Cons (Event p) x xt)
      → time_ordered p ys
      → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge_into p x xt (trace_merge p xt) ys))
        (trace_merge p (trace_window p lo hi (Cons (Event p) x xt)) (trace_window p lo hi ys)) =
  match ys {
    Nil ↦
      λsx.
        λsy.
          sym
            (List (Event p))
            (trace_merge p (trace_window p lo hi (Cons (Event p) x xt)) (Nil (Event p)))
            (trace_window p lo hi (Cons (Event p) x xt))
            (trace_merge_nil_right p (trace_window p lo hi (Cons (Event p) x xt)));
    Cons y yt ↦
      λsx.
        λsy.
          trace_window_merge_into_cons
            p
            lo
            hi
            x
            xt
            ih
            y
            yt
            sx
            sy
            (trace_window_merge_into
              p
              lo
              hi
              x
              xt
              ih
              yt
              sx
              (list_sorted_tail (Event p) (event_before p) y yt sy))
            (event_before p x y)
            Refl
  }

theorem trace_window_merge_into_cons
      (p : Type)
      (lo : Nat)
      (hi : Nat)
      (x : Event p)
      (xt : List (Event p))
      (ih : (zs : List (Event p))
        → time_ordered
        p
        zs
        → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge p xt zs))
        (trace_merge p (trace_window p lo hi xt) (trace_window p lo hi zs)))
      (y : Event p)
      (yt : List (Event p))
      (sx : time_ordered p (Cons (Event p) x xt))
      (sy : time_ordered p (Cons (Event p) y yt))
      (inner : Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge_into p x xt (trace_merge p xt) yt))
        (trace_merge p (trace_window p lo hi (Cons (Event p) x xt)) (trace_window p lo hi yt)))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → Equal
        (List (Event p))
        (trace_window
          p
          lo
          hi
          (trace_merge_into p x xt (trace_merge p xt) (Cons (Event p) y yt)))
        (trace_merge
          p
          (trace_window p lo hi (Cons (Event p) x xt))
          (trace_window p lo hi (Cons (Event p) y yt))) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (trace_window
                p
                lo
                hi
                (match choice {
                  True ↦ Cons (Event p) x (trace_merge p xt (Cons (Event p) y yt));
                  False ↦ Cons (Event p) y (trace_merge_into p x xt (trace_merge p xt) yt)
                }))
              (trace_merge
                p
                (trace_window p lo hi (Cons (Event p) x xt))
                (trace_window p lo hi (Cons (Event p) y yt))))
          (trace_window_merge_left_head
            p
            lo
            hi
            x
            xt
            (trace_merge p xt (Cons (Event p) y yt))
            (trace_window p lo hi (Cons (Event p) y yt))
            (ih (Cons (Event p) y yt) sy)
            (filter_starts_after
              (Event p)
              (event_before p)
              (event_before_trans p)
              (event_in_window p lo hi)
              x
              (Cons (Event p) y yt)
              (list_sorted_cons
                (Event p)
                (event_before p)
                x
                (Cons (Event p) y yt)
                sy
                comparison))
            (event_in_window p lo hi x)
            Refl)
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (trace_window
                p
                lo
                hi
                (match choice {
                  True ↦ Cons (Event p) x (trace_merge p xt (Cons (Event p) y yt));
                  False ↦ Cons (Event p) y (trace_merge_into p x xt (trace_merge p xt) yt)
                }))
              (trace_merge
                p
                (trace_window p lo hi (Cons (Event p) x xt))
                (trace_window p lo hi (Cons (Event p) y yt))))
          (trace_window_merge_right_head
            p
            lo
            hi
            y
            (trace_merge_into p x xt (trace_merge p xt) yt)
            yt
            (trace_window p lo hi (Cons (Event p) x xt))
            inner
            (trace_window_head_after p lo hi x xt y sx comparison)
            (event_in_window p lo hi y)
            Refl)
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_window_merge_left_head
      (p : Type)
      (lo : Nat)
      (hi : Nat)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p))
      (zs : List (Event p))
      (ih : Equal
        (List (Event p))
        (trace_window p lo hi rest)
        (trace_merge p (trace_window p lo hi xt) zs))
      (after : starts_after (Event p) (event_before p) x zs)
      (decision : Bool)
    : Equal Bool (event_in_window p lo hi x) decision
      → Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) x rest))
        (trace_merge p (trace_window p lo hi (Cons (Event p) x xt)) zs) =
  match decision {
    True ↦
      λkept.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) x (trace_window p lo hi rest);
                False ↦ trace_window p lo hi rest
              })
              (trace_merge
                p
                (match choice {
                  True ↦ Cons (Event p) x (trace_window p lo hi xt);
                  False ↦ trace_window p lo hi xt
                })
                zs))
          (trans
            (List (Event p))
            (Cons (Event p) x (trace_window p lo hi rest))
            (Cons (Event p) x (trace_merge p (trace_window p lo hi xt) zs))
            (trace_merge p (Cons (Event p) x (trace_window p lo hi xt)) zs)
            (cong
              (List (Event p))
              (List (Event p))
              (trace_window p lo hi rest)
              (trace_merge p (trace_window p lo hi xt) zs)
              (λtail. Cons (Event p) x tail)
              ih)
            (sym
              (List (Event p))
              (trace_merge p (Cons (Event p) x (trace_window p lo hi xt)) zs)
              (Cons (Event p) x (trace_merge p (trace_window p lo hi xt) zs))
              (trace_merge_cons_left p x (trace_window p lo hi xt) zs after)))
          (sym Bool (event_in_window p lo hi x) True kept);
    False ↦
      λdropped.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) x (trace_window p lo hi rest);
                False ↦ trace_window p lo hi rest
              })
              (trace_merge
                p
                (match choice {
                  True ↦ Cons (Event p) x (trace_window p lo hi xt);
                  False ↦ trace_window p lo hi xt
                })
                zs))
          ih
          (sym Bool (event_in_window p lo hi x) False dropped)
  }

theorem trace_window_merge_right_head
      (p : Type)
      (lo : Nat)
      (hi : Nat)
      (y : Event p)
      (rest : List (Event p))
      (yt : List (Event p))
      (ws : List (Event p))
      (ih : Equal
        (List (Event p))
        (trace_window p lo hi rest)
        (trace_merge p ws (trace_window p lo hi yt)))
      (after : trace_head_after p y ws)
      (decision : Bool)
    : Equal Bool (event_in_window p lo hi y) decision
      → Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) y rest))
        (trace_merge p ws (trace_window p lo hi (Cons (Event p) y yt))) =
  match decision {
    True ↦
      λkept.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) y (trace_window p lo hi rest);
                False ↦ trace_window p lo hi rest
              })
              (trace_merge
                p
                ws
                (match choice {
                  True ↦ Cons (Event p) y (trace_window p lo hi yt);
                  False ↦ trace_window p lo hi yt
                })))
          (trans
            (List (Event p))
            (Cons (Event p) y (trace_window p lo hi rest))
            (Cons (Event p) y (trace_merge p ws (trace_window p lo hi yt)))
            (trace_merge p ws (Cons (Event p) y (trace_window p lo hi yt)))
            (cong
              (List (Event p))
              (List (Event p))
              (trace_window p lo hi rest)
              (trace_merge p ws (trace_window p lo hi yt))
              (λtail. Cons (Event p) y tail)
              ih)
            (sym
              (List (Event p))
              (trace_merge p ws (Cons (Event p) y (trace_window p lo hi yt)))
              (Cons (Event p) y (trace_merge p ws (trace_window p lo hi yt)))
              (trace_merge_cons_right p ws y (trace_window p lo hi yt) after)))
          (sym Bool (event_in_window p lo hi y) True kept);
    False ↦
      λdropped.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) y (trace_window p lo hi rest);
                False ↦ trace_window p lo hi rest
              })
              (trace_merge
                p
                ws
                (match choice {
                  True ↦ Cons (Event p) y (trace_window p lo hi yt);
                  False ↦ trace_window p lo hi yt
                })))
          ih
          (sym Bool (event_in_window p lo hi y) False dropped)
  }

fn trace_head_after (p : Type) (y : Event p) (zs : List (Event p)) : Prop =
  match zs {
    Nil ↦ Top;
    Cons z zt ↦ Equal Bool (event_before p z y) False
  }

theorem trace_merge_nil_right
      (p : Type) (xs : List (Event p))
    : Equal (List (Event p)) (trace_merge p xs (Nil (Event p))) xs =
  match xs {
    Nil ↦ Proved;
    Cons x xt ↦ Refl
  }

theorem trace_merge_cons_left
      (p : Type) (x : Event p) (a : List (Event p)) (zs : List (Event p))
    : starts_after (Event p) (event_before p) x zs
      → Equal
        (List (Event p))
        (trace_merge p (Cons (Event p) x a) zs)
        (Cons (Event p) x (trace_merge p a zs)) =
  match zs {
    Nil ↦
      λafter.
        cong
          (List (Event p))
          (List (Event p))
          a
          (trace_merge p a (Nil (Event p)))
          (λtail. Cons (Event p) x tail)
          (sym
            (List (Event p))
            (trace_merge p a (Nil (Event p)))
            a
            (trace_merge_nil_right p a));
    Cons z zt ↦
      λafter.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) x (trace_merge p a (Cons (Event p) z zt));
                False ↦ Cons (Event p) z (trace_merge_into p x a (trace_merge p a) zt)
              })
              (Cons (Event p) x (trace_merge p a (Cons (Event p) z zt))))
          (sym
            (List (Event p))
            (Cons (Event p) x (trace_merge p a (Cons (Event p) z zt)))
            (Cons (Event p) x (trace_merge p a (Cons (Event p) z zt)))
            Refl)
          (sym Bool (event_before p x z) True after)
  }

theorem trace_merge_cons_right
      (p : Type) (zs : List (Event p)) (y : Event p) (b : List (Event p))
    : trace_head_after p y zs
      → Equal
        (List (Event p))
        (trace_merge p zs (Cons (Event p) y b))
        (Cons (Event p) y (trace_merge p zs b)) =
  match zs {
    Nil ↦ λafter. Refl;
    Cons z zt ↦
      λafter.
        J
          (λchoice _.
            Equal
              (List (Event p))
              (match choice {
                True ↦ Cons (Event p) z (trace_merge p zt (Cons (Event p) y b));
                False ↦ Cons (Event p) y (trace_merge_into p z zt (trace_merge p zt) b)
              })
              (Cons (Event p) y (trace_merge_into p z zt (trace_merge p zt) b)))
          (sym
            (List (Event p))
            (Cons (Event p) y (trace_merge_into p z zt (trace_merge p zt) b))
            (Cons (Event p) y (trace_merge_into p z zt (trace_merge p zt) b))
            Refl)
          (sym Bool (event_before p z y) False after)
  }

theorem trace_window_head_after
      (p : Type) (lo : Nat) (hi : Nat) (x : Event p) (xt : List (Event p)) (y : Event p)
    : time_ordered p (Cons (Event p) x xt)
      → Equal Bool (event_before p x y) False
      → trace_head_after p y (trace_window p lo hi (Cons (Event p) x xt)) =
  λsx.
    λbeyond.
      trace_window_head_after_decide
        p
        lo
        hi
        x
        xt
        y
        beyond
        (trace_head_after_from_start
          p
          x
          y
          (trace_window p lo hi xt)
          (filter_starts_after
            (Event p)
            (event_before p)
            (event_before_trans p)
            (event_in_window p lo hi)
            x
            xt
            sx)
          beyond)
        (event_in_window p lo hi x)
        Refl

theorem trace_window_head_after_decide
      (p : Type)
      (lo : Nat)
      (hi : Nat)
      (x : Event p)
      (xt : List (Event p))
      (y : Event p)
      (beyond : Equal Bool (event_before p x y) False)
      (later : trace_head_after p y (trace_window p lo hi xt))
      (decision : Bool)
    : Equal Bool (event_in_window p lo hi x) decision
      → trace_head_after p y (trace_window p lo hi (Cons (Event p) x xt)) =
  match decision {
    True ↦
      λkept.
        J
          (λchoice _.
            trace_head_after
              p
              y
              (match choice {
                True ↦ Cons (Event p) x (trace_window p lo hi xt);
                False ↦ trace_window p lo hi xt
              }))
          beyond
          (sym Bool (event_in_window p lo hi x) True kept);
    False ↦
      λdropped.
        J
          (λchoice _.
            trace_head_after
              p
              y
              (match choice {
                True ↦ Cons (Event p) x (trace_window p lo hi xt);
                False ↦ trace_window p lo hi xt
              }))
          later
          (sym Bool (event_in_window p lo hi x) False dropped)
  }

theorem trace_head_after_from_start
      (p : Type) (x : Event p) (y : Event p) (zs : List (Event p))
    : starts_after (Event p) (event_before p) x zs
      → Equal Bool (event_before p x y) False
      → trace_head_after p y zs =
  match zs {
    Nil ↦ λafter. λbeyond. Proved;
    Cons z zt ↦
      λafter. λbeyond. trace_head_after_decide p x y z after beyond (event_before p z y) Refl
  }

theorem trace_head_after_decide
      (p : Type)
      (x : Event p)
      (y : Event p)
      (z : Event p)
      (xz : Equal Bool (event_before p x z) True)
      (beyond : Equal Bool (event_before p x y) False)
      (decision : Bool)
    : Equal Bool (event_before p z y) decision → Equal Bool (event_before p z y) False =
  match decision {
    True ↦
      λzy.
        absurd
          (trans
            Bool
            True
            (event_before p x y)
            False
            (sym Bool (event_before p x y) True (event_before_trans p x z y xz zy))
            beyond);
    False ↦ λzy. zy
  }

pub theorem trace_merge_member_sound
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e (trace_merge p xs ys)
      → ‖ Or (list_elem (Event p) e xs) (list_elem (Event p) e ys) ‖ =
  match xs {
    Nil ↦
      λys.
        λmember.
          trunc_intro
            (Inr (list_elem (Event p) e (Nil (Event p))) (list_elem (Event p) e ys) member);
    Cons x xt ↦
      λys.
        trace_merge_into_member_sound
          p
          e
          x
          xt
          (trace_merge p xt)
          (λzs. trace_merge_member_sound p e xt zs)
          ys
  }

pub theorem trace_merge_member_left
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e xs
      → list_elem (Event p) e (trace_merge p xs ys) =
  match xs {
    Nil ↦ λys. λmember. absurd member;
    Cons x xt ↦
      λys.
        trace_merge_into_member_left
          p
          e
          x
          xt
          (trace_merge p xt)
          (λzs. trace_merge_member_left p e xt zs)
          ys
  }

pub theorem trace_merge_member_right
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e ys
      → list_elem (Event p) e (trace_merge p xs ys) =
  match xs {
    Nil ↦ λys. λmember. member;
    Cons x xt ↦
      λys.
        trace_merge_into_member_right
          p
          e
          x
          xt
          (trace_merge p xt)
          (λzs. trace_merge_member_right p e xt zs)
          ys
  }

theorem trace_merge_into_member_sound_cons
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_sound : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        (rest zs)
        → ‖ Or
        (list_elem (Event p) e xt)
        (list_elem (Event p) e zs) ‖)
      (y : Event p)
      (yt : List (Event p))
      (ih : list_elem
        (Event p)
        e
        (trace_merge_into p x xt rest yt)
        → ‖ Or
        (list_elem (Event p) e (Cons (Event p) x xt))
        (list_elem (Event p) e yt) ‖)
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → list_elem (Event p) e (trace_merge_into p x xt rest (Cons (Event p) y yt))
      → ‖ Or
        (list_elem (Event p) e (Cons (Event p) x xt))
        (list_elem (Event p) e (Cons (Event p) y yt)) ‖ =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              })
            → ‖ Or
              (list_elem (Event p) e (Cons (Event p) x xt))
              (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
          (λmember.
            elim_trunc
              (‖ Or
                (list_elem (Event p) e (Cons (Event p) x xt))
                (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
              (λcase.
                match case {
                  Inl same ↦
                    trunc_intro
                      (Inl
                        (list_elem (Event p) e (Cons (Event p) x xt))
                        (list_elem (Event p) e (Cons (Event p) y yt))
                        (trunc_intro
                          (Inl (Equal (Event p) e x) (list_elem (Event p) e xt) same)));
                  Inr later ↦
                    elim_trunc
                      (‖ Or
                        (list_elem (Event p) e (Cons (Event p) x xt))
                        (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
                      (λinner.
                        match inner {
                          Inl in_xt ↦
                            trunc_intro
                              (Inl
                                (list_elem (Event p) e (Cons (Event p) x xt))
                                (list_elem (Event p) e (Cons (Event p) y yt))
                                (list_elem_later (Event p) e x xt in_xt));
                          Inr in_ys ↦
                            trunc_intro
                              (Inr
                                (list_elem (Event p) e (Cons (Event p) x xt))
                                (list_elem (Event p) e (Cons (Event p) y yt))
                                in_ys)
                        })
                      (rest_sound (Cons (Event p) y yt) later)
                })
              member)
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              })
            → ‖ Or
              (list_elem (Event p) e (Cons (Event p) x xt))
              (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
          (λmember.
            elim_trunc
              (‖ Or
                (list_elem (Event p) e (Cons (Event p) x xt))
                (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
              (λcase.
                match case {
                  Inl same ↦
                    trunc_intro
                      (Inr
                        (list_elem (Event p) e (Cons (Event p) x xt))
                        (list_elem (Event p) e (Cons (Event p) y yt))
                        (trunc_intro
                          (Inl (Equal (Event p) e y) (list_elem (Event p) e yt) same)));
                  Inr later ↦
                    elim_trunc
                      (‖ Or
                        (list_elem (Event p) e (Cons (Event p) x xt))
                        (list_elem (Event p) e (Cons (Event p) y yt)) ‖)
                      (λinner.
                        match inner {
                          Inl in_x ↦
                            trunc_intro
                              (Inl
                                (list_elem (Event p) e (Cons (Event p) x xt))
                                (list_elem (Event p) e (Cons (Event p) y yt))
                                in_x);
                          Inr in_yt ↦
                            trunc_intro
                              (Inr
                                (list_elem (Event p) e (Cons (Event p) x xt))
                                (list_elem (Event p) e (Cons (Event p) y yt))
                                (list_elem_later (Event p) e y yt in_yt))
                        })
                      (ih later)
                })
              member)
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_member_sound
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_sound : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        (rest zs)
        → ‖ Or
        (list_elem (Event p) e xt)
        (list_elem (Event p) e zs) ‖)
      (ys : List (Event p))
    : list_elem (Event p) e (trace_merge_into p x xt rest ys)
      → ‖ Or (list_elem (Event p) e (Cons (Event p) x xt)) (list_elem (Event p) e ys) ‖ =
  match ys {
    Nil ↦
      λmember.
        trunc_intro
          (Inl
            (list_elem (Event p) e (Cons (Event p) x xt))
            (list_elem (Event p) e (Nil (Event p)))
            member);
    Cons y yt ↦
      trace_merge_into_member_sound_cons
        p
        e
        x
        xt
        rest
        rest_sound
        y
        yt
        (trace_merge_into_member_sound p e x xt rest rest_sound yt)
        (event_before p x y)
        Refl
  }

theorem trace_merge_into_member_left_cons
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_left : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        xt
        → list_elem
        (Event p)
        e
        (rest zs))
      (y : Event p)
      (yt : List (Event p))
      (ih : list_elem
        (Event p)
        e
        (Cons (Event p) x xt)
        → list_elem
        (Event p)
        e
        (trace_merge_into p x xt rest yt))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → list_elem (Event p) e (Cons (Event p) x xt)
      → list_elem (Event p) e (trace_merge_into p x xt rest (Cons (Event p) y yt)) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            list_elem (Event p) e (Cons (Event p) x xt)
            → list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (λmember.
            elim_trunc
              (list_elem (Event p) e (Cons (Event p) x (rest (Cons (Event p) y yt))))
              (λcase.
                match case {
                  Inl same ↦
                    trunc_intro
                      (Inl
                        (Equal (Event p) e x)
                        (list_elem (Event p) e (rest (Cons (Event p) y yt)))
                        same);
                  Inr later ↦
                    list_elem_later
                      (Event p)
                      e
                      x
                      (rest (Cons (Event p) y yt))
                      (rest_left (Cons (Event p) y yt) later)
                })
              member)
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            list_elem (Event p) e (Cons (Event p) x xt)
            → list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (λmember. list_elem_later (Event p) e y (trace_merge_into p x xt rest yt) (ih member))
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_member_left
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_left : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        xt
        → list_elem
        (Event p)
        e
        (rest zs))
      (ys : List (Event p))
    : list_elem (Event p) e (Cons (Event p) x xt)
      → list_elem (Event p) e (trace_merge_into p x xt rest ys) =
  match ys {
    Nil ↦ λmember. member;
    Cons y yt ↦
      trace_merge_into_member_left_cons
        p
        e
        x
        xt
        rest
        rest_left
        y
        yt
        (trace_merge_into_member_left p e x xt rest rest_left yt)
        (event_before p x y)
        Refl
  }

theorem trace_merge_into_member_right_cons
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_right : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        zs
        → list_elem
        (Event p)
        e
        (rest zs))
      (y : Event p)
      (yt : List (Event p))
      (ih : list_elem (Event p) e yt → list_elem (Event p) e (trace_merge_into p x xt rest yt))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → list_elem (Event p) e (Cons (Event p) y yt)
      → list_elem (Event p) e (trace_merge_into p x xt rest (Cons (Event p) y yt)) =
  match decision {
    True ↦
      λcomparison.
        J
          (λchoice _.
            list_elem (Event p) e (Cons (Event p) y yt)
            → list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (λmember.
            list_elem_later
              (Event p)
              e
              x
              (rest (Cons (Event p) y yt))
              (rest_right (Cons (Event p) y yt) member))
          (sym Bool (event_before p x y) True comparison);
    False ↦
      λcomparison.
        J
          (λchoice _.
            list_elem (Event p) e (Cons (Event p) y yt)
            → list_elem
              (Event p)
              e
              (match choice {
                True ↦ Cons (Event p) x (rest (Cons (Event p) y yt));
                False ↦ Cons (Event p) y (trace_merge_into p x xt rest yt)
              }))
          (λmember.
            elim_trunc
              (list_elem (Event p) e (Cons (Event p) y (trace_merge_into p x xt rest yt)))
              (λcase.
                match case {
                  Inl same ↦
                    trunc_intro
                      (Inl
                        (Equal (Event p) e y)
                        (list_elem (Event p) e (trace_merge_into p x xt rest yt))
                        same);
                  Inr later ↦
                    list_elem_later (Event p) e y (trace_merge_into p x xt rest yt) (ih later)
                })
              member)
          (sym Bool (event_before p x y) False comparison)
  }

theorem trace_merge_into_member_right
      (p : Type)
      (e : Event p)
      (x : Event p)
      (xt : List (Event p))
      (rest : List (Event p) → List (Event p))
      (rest_right : (zs : List (Event p))
        → list_elem
        (Event p)
        e
        zs
        → list_elem
        (Event p)
        e
        (rest zs))
      (ys : List (Event p))
    : list_elem (Event p) e ys → list_elem (Event p) e (trace_merge_into p x xt rest ys) =
  match ys {
    Nil ↦ λmember. absurd member;
    Cons y yt ↦
      trace_merge_into_member_right_cons
        p
        e
        x
        xt
        rest
        rest_right
        y
        yt
        (trace_merge_into_member_right p e x xt rest rest_right yt)
        (event_before p x y)
        Refl
  }

theorem trace_window_merge_member_sound
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p)) (ys : List (Event p)) (e : Event p)
    : list_elem (Event p) e (trace_window p lo hi (trace_merge p xs ys))
      → ‖ Or (list_elem (Event p) e xs) (list_elem (Event p) e ys) ‖ =
  λmember.
    trace_merge_member_sound
      p
      e
      xs
      ys
      (and_fst
        (list_elem (Event p) e (trace_merge p xs ys))
        (And
          (Equal Bool (leq_nat lo (event_time p e)) True)
          (Equal Bool (lt_nat (event_time p e) hi) True))
        (trace_window_member_sound p lo hi (trace_merge p xs ys) e member))
```

## 3. Using it

The exported constructor lets an external client build two equal-time
events with different payloads. The left event is emitted first:

```ken example
const event_a : Event Bool = MkEvent Bool (Suc Zero) True

const event_b : Event Bool = MkEvent Bool (Suc Zero) False

theorem event_left_tie_example
    : Equal
        (List (Event Bool))
        (trace_merge
          Bool
          (Cons (Event Bool) event_a (Nil (Event Bool)))
          (Cons (Event Bool) event_b (Nil (Event Bool))))
        (Cons (Event Bool) event_a (Cons (Event Bool) event_b (Nil (Event Bool)))) =
  Refl
```

## 4. Laws & proofs

Filtering preserves order by transitivity of the timestamp comparison even
when intervening entries disappear. The generic sorted-filter proof and its
sorted-cons, head and tail bridges are reused from the collections provider.

Window membership reuses generic filter soundness and completeness from the
collections provider. Soundness projects an input-member witness and the
predicate fact, then separates the lower and strict upper timestamp bounds.
The filter evidence is shared rather than reproved for each projection.
Completeness joins the two bounds using checked Boolean conjunction and
applies generic filter completeness. Membership is Ω-valued even for a payload
without decidable equality; neither law requires the trace to be ordered.

The merge-order proof considers which input head precedes the other; totality
of natural-number order supplies the reverse comparison when the left
comparison is false. Head-order evidence is carried through recursive merge.

Occurrence count needs no ordering premise. Its induction splits on the two
head-comparison outcomes, using generic `count_cons` accounting. The private
right-addition shuffle inducts on one addend using the checked successor law;
the right-head count proof branches on the classifier and lifts its premise
through `Suc`. The classifier is arbitrary: it may ignore its query and need
not decide payload equality.

Window/merge distribution inducts over the left trace and carries a
right-trace induction in the private head-insertion case. Generic filtered
order preserves the comparison between a kept head and its tail, including
across discarded events. When the heads compare in the opposite direction,
that order evidence lets the right head lead both windowed arrangements.
The two input-order premises preserve the head comparisons; no ordering
premise is needed merely to compute either operation.

Merge membership instead needs no ordering premise. Three checked laws prove
that a member of the merge came from at least one input and that a member of
either input remains in the merge. Their private recursive helpers follow the
head-insertion branch for each input; a named comparison decision transports
membership through both Boolean outcomes. The proofs reuse the collections
provider's `list_elem_later` for a member behind a retained head, rather than
append laws for a merge that is not append. A private consequence combines
window-membership soundness with merge-membership soundness: a windowed-merge
member came from at least one input. It does not equate windowed lists for
unordered inputs or count duplicate occurrences.

## 5. Design notes

`time_ordered` is a proposition over a plain list, not a field of `Event` or
of a separate trace wrapper. Unordered inputs remain legal; only the
order-preservation laws ask for ordered inputs. The timestamp order is
non-decreasing, admitting tied events. `trace_window` is half-open; equal or
inverted bounds select no events. Membership in a window supplies two separate
facts: its timestamp meets the inclusive lower and strict upper bound. The
merge's tie rule is part of its
computation, not a consequence of the count law. No clock, duration, signed
time, infinite stream, or causality property is provided here. Membership
claims occurrence without counting duplicates; the separate count law concerns
merging, not window multiplicity.

## 6. References

- **Wikipedia** — [Merge algorithm](https://en.wikipedia.org/wiki/Merge_algorithm)
  — orientation on merging two already ordered inputs; this package also
  defines a result for unordered inputs by comparing current heads.
- **Ken specification** —
  [Finite event traces](../../../../spec/50-stdlib/69-time-and-events.md)
  — the carrier, half-open boundary, left-bias and nine checked law contracts.
- **Collections provider** —
  [Derived collections](../../Data/Collections/Derived.ken.md)
  — generic filtering, list membership, sorted bridges and counting.

## 7. Trust & derivation

1. **Public API.** `Event`, `MkEvent`, its two projections, the time
   comparison, ordering predicate, window predicate, `trace_window`,
   `trace_merge`, and the nine laws form the package surface.
2. **Source map.** [Definition](#2-definition) contains every public
   declaration and private support; [Using it](#3-using-it) shows an
   observable tie; [Laws & proofs](#4-laws--proofs) traces the proof families.
3. **Derivation.** `Nat` and `List` are existing carriers; windowing uses the
   derived `filter`; merging is structurally recursive on two lists; count
   preservation uses derived `count` and checked addition laws. Generic list
   lemmas are owned by the collections provider, not copied into this package.
   Window membership instantiates its two generic filter proofs and projects
   both half-open timestamp bounds. Window/merge distribution also uses
   the public filtered-head-order proof from the collections provider.
   Merge membership reuses `list_elem_later` and ordinary truncated `Or`;
   the one-way window-of-merge consequence composes the two checked soundness
   laws and remains private.
4. **Trusted-base delta.** Zero: all nine exported laws have checked proof
   bodies, and this package adds no `Axiom` or primitive. Imported logical,
   numeric and collection providers retain their inherited trust.
5. **Validation.** The
   [trace seed](../../../../conformance/stdlib/time-and-events/seed-event-trace.md),
   [window-membership seed](../../../../conformance/stdlib/time-and-events/seed-window-membership.md),
   [distribution seed](../../../../conformance/stdlib/time-and-events/seed-window-merge-distribution.md)
   and [merge-membership seed](../../../../conformance/stdlib/time-and-events/seed-merge-membership.md)
   supply independent closed values and refused claims; these clients
   complement, not replace, the generic proofs. A package test runs the
   merge-membership seed, including its pinned refusal; this is not an
   official conformance-runner or CI claim.
