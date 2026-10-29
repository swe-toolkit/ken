# Finite event traces

Finite events with natural-number times support half-open windows and a
left-biased two-list merge with checked ordering, count, membership,
window/merge distribution, window composition and an adjacent-window
split. Merge membership holds even for unordered inputs.

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
lemmas. The window/merge distribution and composition laws, the three merge
membership laws, merge associativity and the adjacent-window split each lead
their private lemmas later in the source. Windowing includes the lower bound
but excludes the upper; a merge tie favors the left input without examining
payloads. Membership uses Ω-valued `list_elem`, so neither a payload equality
decision nor an ordering premise is needed.

```ken
import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)

import Core.Classes.LawfulClasses (IsTrue, and_true, bool_and, bool_or, leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Core.Logic.Or (Or, Inl, Inr)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat, max, min)

import Data.Collections.Derived (is_sorted)

import Data.Collections.Derived
  (filter,
    list_append,
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

pub theorem trace_window_window
      (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat) (tr : List (Event p))
    : Equal
        (List (Event p))
        (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
        (trace_window p (max lo1 lo2) (min hi1 hi2) tr) =
  window_filter_fuse
    (Event p)
    (event_in_window p lo1 hi1)
    (event_in_window p lo2 hi2)
    (event_in_window p (max lo1 lo2) (min hi1 hi2))
    (λe. event_in_window_inter p lo1 hi1 lo2 hi2 e)
    tr

pub theorem trace_window_split
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr
      → Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid hi) True
      → Equal
        (List (Event p))
        (trace_window p lo hi tr)
        (list_append (Event p) (trace_window p lo mid tr) (trace_window p mid hi tr)) =
  match tr {
    Nil ↦ λordered. λlo_mid. λmid_hi. Proved;
    Cons h t ↦
      λordered.
        λlo_mid.
          λmid_hi.
            trace_window_split_cons
              p
              lo
              mid
              hi
              h
              t
              ordered
              lo_mid
              mid_hi
              (trace_window_split
                p
                lo
                mid
                hi
                t
                (list_sorted_tail (Event p) (event_before p) h t ordered)
                lo_mid
                mid_hi)
              (lt_nat (event_time p h) mid)
              Refl
  }

pub theorem trace_window_append
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p)) (ys : List (Event p))
    : Equal
        (List (Event p))
        (trace_window p lo hi (list_append (Event p) xs ys))
        (list_append (Event p) (trace_window p lo hi xs) (trace_window p lo hi ys)) =
  (proof append for filter) (Event p) (event_in_window p lo hi) xs ys

pub theorem trace_merge_nil_left
      (p : Type) (ys : List (Event p))
    : Equal (List (Event p)) (trace_merge p (Nil (Event p)) ys) ys =
  Refl

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

pub theorem trace_merge_nil_right
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

pub theorem trace_merge_assoc
      (p : Type) (xs : List (Event p)) (ys : List (Event p)) (zs : List (Event p))
    : Equal
        (List (Event p))
        (trace_merge p (trace_merge p xs ys) zs)
        (trace_merge p xs (trace_merge p ys zs)) =
  match xs {
    Nil ↦ Refl;
    Cons x a ↦ trace_merge_assoc_cons p x a (λvs. λws. trace_merge_assoc p a vs ws) ys zs
  }

theorem trace_merge_assoc_cons
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (ih_a : (vs : List (Event p))
        → (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p a vs) ws)
        (trace_merge p a (trace_merge p vs ws)))
      (ys : List (Event p))
      (zs : List (Event p))
    : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) ys) zs)
        (trace_merge p (Cons (Event p) x a) (trace_merge p ys zs)) =
  match ys {
    Nil ↦ Refl;
    Cons y b ↦
      trace_merge_assoc_cons_cons
        p
        x
        a
        ih_a
        y
        b
        (λws. trace_merge_assoc_cons p x a ih_a b ws)
        zs
  }

theorem trace_merge_assoc_cons_cons
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (ih_a : (vs : List (Event p))
        → (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p a vs) ws)
        (trace_merge p a (trace_merge p vs ws)))
      (y : Event p)
      (b : List (Event p))
      (ih_b : (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) b) ws)
        (trace_merge p (Cons (Event p) x a) (trace_merge p b ws)))
      (zs : List (Event p))
    : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) zs)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) zs)) =
  match zs {
    Nil ↦ trace_merge_nil_right p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b));
    Cons z c ↦
      trace_merge_assoc_step
        p
        x
        a
        ih_a
        y
        b
        ih_b
        z
        c
        (trace_merge_assoc_cons_cons p x a ih_a y b ih_b c)
        (event_before p x y)
        Refl
  }

theorem trace_merge_assoc_step
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (ih_a : (vs : List (Event p))
        → (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p a vs) ws)
        (trace_merge p a (trace_merge p vs ws)))
      (y : Event p)
      (b : List (Event p))
      (ih_b : (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) b) ws)
        (trace_merge p (Cons (Event p) x a) (trace_merge p b ws)))
      (z : Event p)
      (c : List (Event p))
      (ih_c : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (decision : Bool)
    : Equal Bool (event_before p x y) decision
      → Equal
        (List (Event p))
        (trace_merge
          p
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          (Cons (Event p) z c))
        (trace_merge
          p
          (Cons (Event p) x a)
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))) =
  match decision {
    True ↦
      λxy. trace_merge_assoc_left_first p x a ih_a y b z c ih_c xy (event_before p x z) Refl;
    False ↦
      λxy. trace_merge_assoc_right_first p x a y b ih_b z c ih_c xy (event_before p y z) Refl
  }

theorem trace_merge_assoc_left_first
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (ih_a : (vs : List (Event p))
        → (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p a vs) ws)
        (trace_merge p a (trace_merge p vs ws)))
      (y : Event p)
      (b : List (Event p))
      (z : Event p)
      (c : List (Event p))
      (ih_c : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (xy : Equal Bool (event_before p x y) True)
      (decision : Bool)
    : Equal Bool (event_before p x z) decision
      → Equal
        (List (Event p))
        (trace_merge
          p
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          (Cons (Event p) z c))
        (trace_merge
          p
          (Cons (Event p) x a)
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))) =
  match decision {
    True ↦
      λxz.
        trans
          (List (Event p))
          (trace_merge
            p
            (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
            (Cons (Event p) z c))
          (Cons
            (Event p)
            x
            (trace_merge p a (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))))
          (trace_merge
            p
            (Cons (Event p) x a)
            (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
          (trans
            (List (Event p))
            (trace_merge
              p
              (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
              (Cons (Event p) z c))
            (Cons
              (Event p)
              x
              (trace_merge p (trace_merge p a (Cons (Event p) y b)) (Cons (Event p) z c)))
            (Cons
              (Event p)
              x
              (trace_merge p a (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))))
            (trans
              (List (Event p))
              (trace_merge
                p
                (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
                (Cons (Event p) z c))
              (trace_merge
                p
                (Cons (Event p) x (trace_merge p a (Cons (Event p) y b)))
                (Cons (Event p) z c))
              (Cons
                (Event p)
                x
                (trace_merge p (trace_merge p a (Cons (Event p) y b)) (Cons (Event p) z c)))
              (cong
                (List (Event p))
                (List (Event p))
                (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
                (Cons (Event p) x (trace_merge p a (Cons (Event p) y b)))
                (λleft. trace_merge p left (Cons (Event p) z c))
                (trace_merge_cons_left p x a (Cons (Event p) y b) xy))
              (trace_merge_cons_left
                p
                x
                (trace_merge p a (Cons (Event p) y b))
                (Cons (Event p) z c)
                xz))
            (cong
              (List (Event p))
              (List (Event p))
              (trace_merge p (trace_merge p a (Cons (Event p) y b)) (Cons (Event p) z c))
              (trace_merge p a (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
              (λtail. Cons (Event p) x tail)
              (ih_a (Cons (Event p) y b) (Cons (Event p) z c))))
          (sym
            (List (Event p))
            (trace_merge
              p
              (Cons (Event p) x a)
              (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
            (Cons
              (Event p)
              x
              (trace_merge p a (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))))
            (trace_merge_cons_left
              p
              x
              a
              (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))
              (trace_merge_starts_after p x (Cons (Event p) y b) (Cons (Event p) z c) xy xz)));
    False ↦
      λxz.
        trace_merge_assoc_right_last
          p
          x
          a
          y
          b
          z
          c
          ih_c
          (Cons (Event p) x (trace_merge p a (Cons (Event p) y b)))
          (trace_merge_cons_left p x a (Cons (Event p) y b) xy)
          (trace_merge_cons_right
            p
            (Cons (Event p) x (trace_merge p a (Cons (Event p) y b)))
            z
            c
            xz)
          (trace_head_after_decide p x z y xy xz (event_before p y z) Refl)
          xz
  }

theorem trace_merge_assoc_right_first
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (y : Event p)
      (b : List (Event p))
      (ih_b : (ws : List (Event p))
        → Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) b) ws)
        (trace_merge p (Cons (Event p) x a) (trace_merge p b ws)))
      (z : Event p)
      (c : List (Event p))
      (ih_c : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (xy : Equal Bool (event_before p x y) False)
      (decision : Bool)
    : Equal Bool (event_before p y z) decision
      → Equal
        (List (Event p))
        (trace_merge
          p
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          (Cons (Event p) z c))
        (trace_merge
          p
          (Cons (Event p) x a)
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))) =
  match decision {
    True ↦
      λyz.
        trans
          (List (Event p))
          (trace_merge
            p
            (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
            (Cons (Event p) z c))
          (Cons
            (Event p)
            y
            (trace_merge p (Cons (Event p) x a) (trace_merge p b (Cons (Event p) z c))))
          (trace_merge
            p
            (Cons (Event p) x a)
            (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
          (trans
            (List (Event p))
            (trace_merge
              p
              (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
              (Cons (Event p) z c))
            (Cons
              (Event p)
              y
              (trace_merge p (trace_merge p (Cons (Event p) x a) b) (Cons (Event p) z c)))
            (Cons
              (Event p)
              y
              (trace_merge p (Cons (Event p) x a) (trace_merge p b (Cons (Event p) z c))))
            (trans
              (List (Event p))
              (trace_merge
                p
                (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
                (Cons (Event p) z c))
              (trace_merge
                p
                (Cons (Event p) y (trace_merge p (Cons (Event p) x a) b))
                (Cons (Event p) z c))
              (Cons
                (Event p)
                y
                (trace_merge p (trace_merge p (Cons (Event p) x a) b) (Cons (Event p) z c)))
              (cong
                (List (Event p))
                (List (Event p))
                (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
                (Cons (Event p) y (trace_merge p (Cons (Event p) x a) b))
                (λleft. trace_merge p left (Cons (Event p) z c))
                (trace_merge_cons_right p (Cons (Event p) x a) y b xy))
              (trace_merge_cons_left
                p
                y
                (trace_merge p (Cons (Event p) x a) b)
                (Cons (Event p) z c)
                yz))
            (cong
              (List (Event p))
              (List (Event p))
              (trace_merge p (trace_merge p (Cons (Event p) x a) b) (Cons (Event p) z c))
              (trace_merge p (Cons (Event p) x a) (trace_merge p b (Cons (Event p) z c)))
              (λtail. Cons (Event p) y tail)
              (ih_b (Cons (Event p) z c))))
          (sym
            (List (Event p))
            (trace_merge
              p
              (Cons (Event p) x a)
              (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
            (Cons
              (Event p)
              y
              (trace_merge p (Cons (Event p) x a) (trace_merge p b (Cons (Event p) z c))))
            (trans
              (List (Event p))
              (trace_merge
                p
                (Cons (Event p) x a)
                (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
              (trace_merge
                p
                (Cons (Event p) x a)
                (Cons (Event p) y (trace_merge p b (Cons (Event p) z c))))
              (Cons
                (Event p)
                y
                (trace_merge p (Cons (Event p) x a) (trace_merge p b (Cons (Event p) z c))))
              (cong
                (List (Event p))
                (List (Event p))
                (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))
                (Cons (Event p) y (trace_merge p b (Cons (Event p) z c)))
                (λright. trace_merge p (Cons (Event p) x a) right)
                (trace_merge_cons_left p y b (Cons (Event p) z c) yz))
              (trace_merge_cons_right
                p
                (Cons (Event p) x a)
                y
                (trace_merge p b (Cons (Event p) z c))
                xy)));
    False ↦
      λyz.
        trace_merge_assoc_right_last
          p
          x
          a
          y
          b
          z
          c
          ih_c
          (Cons (Event p) y (trace_merge p (Cons (Event p) x a) b))
          (trace_merge_cons_right p (Cons (Event p) x a) y b xy)
          (trace_merge_cons_right
            p
            (Cons (Event p) y (trace_merge p (Cons (Event p) x a) b))
            z
            c
            yz)
          yz
          (event_before_skip p x y z xy (event_before_flip p y z yz) (event_before p x z) Refl)
  }

theorem trace_merge_assoc_right_last
      (p : Type)
      (x : Event p)
      (a : List (Event p))
      (y : Event p)
      (b : List (Event p))
      (z : Event p)
      (c : List (Event p))
      (ih_c : Equal
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (unfolded : List (Event p))
      (merged_unfolds : Equal
        (List (Event p))
        (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
        unfolded)
      (unfolded_right : Equal
        (List (Event p))
        (trace_merge p unfolded (Cons (Event p) z c))
        (Cons (Event p) z (trace_merge p unfolded c)))
      (yz : Equal Bool (event_before p y z) False)
      (xz : Equal Bool (event_before p x z) False)
    : Equal
        (List (Event p))
        (trace_merge
          p
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          (Cons (Event p) z c))
        (trace_merge
          p
          (Cons (Event p) x a)
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))) =
  trans
    (List (Event p))
    (trace_merge
      p
      (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
      (Cons (Event p) z c))
    (Cons
      (Event p)
      z
      (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
    (trace_merge
      p
      (Cons (Event p) x a)
      (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
    (trans
      (List (Event p))
      (trace_merge
        p
        (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
        (Cons (Event p) z c))
      (Cons
        (Event p)
        z
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c))
      (Cons
        (Event p)
        z
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (trans
        (List (Event p))
        (trace_merge
          p
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          (Cons (Event p) z c))
        (trace_merge p unfolded (Cons (Event p) z c))
        (Cons
          (Event p)
          z
          (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c))
        (cong
          (List (Event p))
          (List (Event p))
          (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
          unfolded
          (λleft. trace_merge p left (Cons (Event p) z c))
          merged_unfolds)
        (trans
          (List (Event p))
          (trace_merge p unfolded (Cons (Event p) z c))
          (Cons (Event p) z (trace_merge p unfolded c))
          (Cons
            (Event p)
            z
            (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c))
          unfolded_right
          (cong
            (List (Event p))
            (List (Event p))
            unfolded
            (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
            (λleft. Cons (Event p) z (trace_merge p left c))
            (sym
              (List (Event p))
              (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b))
              unfolded
              merged_unfolds))))
      (cong
        (List (Event p))
        (List (Event p))
        (trace_merge p (trace_merge p (Cons (Event p) x a) (Cons (Event p) y b)) c)
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c))
        (λtail. Cons (Event p) z tail)
        ih_c))
    (sym
      (List (Event p))
      (trace_merge
        p
        (Cons (Event p) x a)
        (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
      (Cons
        (Event p)
        z
        (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
      (trans
        (List (Event p))
        (trace_merge
          p
          (Cons (Event p) x a)
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c)))
        (trace_merge
          p
          (Cons (Event p) x a)
          (Cons (Event p) z (trace_merge p (Cons (Event p) y b) c)))
        (Cons
          (Event p)
          z
          (trace_merge p (Cons (Event p) x a) (trace_merge p (Cons (Event p) y b) c)))
        (cong
          (List (Event p))
          (List (Event p))
          (trace_merge p (Cons (Event p) y b) (Cons (Event p) z c))
          (Cons (Event p) z (trace_merge p (Cons (Event p) y b) c))
          (λright. trace_merge p (Cons (Event p) x a) right)
          (trace_merge_cons_right p (Cons (Event p) y b) z c yz))
        (trace_merge_cons_right
          p
          (Cons (Event p) x a)
          z
          (trace_merge p (Cons (Event p) y b) c)
          xz)))

theorem event_before_skip
      (p : Type)
      (x : Event p)
      (y : Event p)
      (z : Event p)
      (beyond : Equal Bool (event_before p x y) False)
      (zy : Equal Bool (event_before p z y) True)
      (decision : Bool)
    : Equal Bool (event_before p x z) decision → Equal Bool (event_before p x z) False =
  match decision {
    True ↦
      λxz.
        absurd
          (trans
            Bool
            True
            (event_before p x y)
            False
            (sym Bool (event_before p x y) True (event_before_trans p x z y xz zy))
            beyond);
    False ↦ λxz. xz
  }

theorem leq_nat_max_split
      (a : Nat) (b : Nat) (t : Nat)
    : Equal Bool (leq_nat (max a b) t) (bool_and (leq_nat a t) (leq_nat b t)) =
  match a {
    Zero ↦ Refl;
    Suc a2 ↦
      match b {
        Zero ↦
          match t {
            Zero ↦ Proved;
            Suc t2 ↦
              sym
                Bool
                (bool_and (leq_nat a2 t2) True)
                (leq_nat a2 t2)
                ((proof right_identity for bool_and) (leq_nat a2 t2))
          };
        Suc b2 ↦
          match t {
            Zero ↦ Proved;
            Suc t2 ↦ leq_nat_max_split a2 b2 t2
          }
      }
  }

theorem lt_nat_min_split
      (a : Nat) (b : Nat) (t : Nat)
    : Equal Bool (lt_nat t (min a b)) (bool_and (lt_nat t a) (lt_nat t b)) =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦
      match b {
        Zero ↦
          match t {
            Zero ↦ Proved;
            Suc t2 ↦
              sym
                Bool
                (bool_and (lt_nat t2 a2) False)
                False
                ((proof comm for bool_and) (lt_nat t2 a2) False)
          };
        Suc b2 ↦
          match t {
            Zero ↦ Proved;
            Suc t2 ↦ lt_nat_min_split a2 b2 t2
          }
      }
  }

theorem bool_and_interchange
      (a : Bool) (b : Bool) (c : Bool) (d : Bool)
    : Equal Bool
        (bool_and (bool_and a b) (bool_and c d))
        (bool_and (bool_and a c) (bool_and b d)) =
  match a {
    True ↦
      match b {
        True ↦
          match c {
            True ↦ Refl;
            False ↦ Proved
          };
        False ↦
          match c {
            True ↦
              match d {
                True ↦ Proved;
                False ↦ Proved
              };
            False ↦ Proved
          }
      };
    False ↦ Proved
  }

theorem event_in_window_inter
      (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat) (e : Event p)
    : Equal Bool
        (bool_and (event_in_window p lo1 hi1 e) (event_in_window p lo2 hi2 e))
        (event_in_window p (max lo1 lo2) (min hi1 hi2) e) =
  match e {
    MkEvent t x ↦
      trans
        Bool
        (bool_and
          (bool_and (leq_nat lo1 t) (lt_nat t hi1))
          (bool_and (leq_nat lo2 t) (lt_nat t hi2)))
        (bool_and
          (bool_and (leq_nat lo1 t) (leq_nat lo2 t))
          (bool_and (lt_nat t hi1) (lt_nat t hi2)))
        (bool_and (leq_nat (max lo1 lo2) t) (lt_nat t (min hi1 hi2)))
        (bool_and_interchange (leq_nat lo1 t) (lt_nat t hi1) (leq_nat lo2 t) (lt_nat t hi2))
        (trans
          Bool
          (bool_and
            (bool_and (leq_nat lo1 t) (leq_nat lo2 t))
            (bool_and (lt_nat t hi1) (lt_nat t hi2)))
          (bool_and (leq_nat (max lo1 lo2) t) (bool_and (lt_nat t hi1) (lt_nat t hi2)))
          (bool_and (leq_nat (max lo1 lo2) t) (lt_nat t (min hi1 hi2)))
          (cong
            Bool
            Bool
            (bool_and (leq_nat lo1 t) (leq_nat lo2 t))
            (leq_nat (max lo1 lo2) t)
            (λlower. bool_and lower (bool_and (lt_nat t hi1) (lt_nat t hi2)))
            (sym
              Bool
              (leq_nat (max lo1 lo2) t)
              (bool_and (leq_nat lo1 t) (leq_nat lo2 t))
              (leq_nat_max_split lo1 lo2 t)))
          (cong
            Bool
            Bool
            (bool_and (lt_nat t hi1) (lt_nat t hi2))
            (lt_nat t (min hi1 hi2))
            (λupper. bool_and (leq_nat (max lo1 lo2) t) upper)
            (sym
              Bool
              (lt_nat t (min hi1 hi2))
              (bool_and (lt_nat t hi1) (lt_nat t hi2))
              (lt_nat_min_split hi1 hi2 t))))
  }

fn window_keep (a : Type) (h : a) (rest : List a) (decision : Bool) : List a =
  match decision {
    True ↦ Cons a h rest;
    False ↦ rest
  }

theorem window_filter_fuse_cons
      (a : Type)
      (r : a → Bool)
      (q : a → Bool)
      (s : a → Bool)
      (h : a)
      (t : List a)
      (ih : Equal (List a) (filter a q (filter a r t)) (filter a s t))
      (dr : Bool)
      (dq : Bool)
      (ds : Bool)
    : Equal Bool (q h) dq
      → Equal Bool (bool_and dr dq) ds
      → Equal
        (List a)
        (filter a q (window_keep a h (filter a r t) dr))
        (window_keep a h (filter a s t) ds) =
  match dr {
    True ↦
      match dq {
        True ↦
          match ds {
            True ↦
              λkept.
                λjoined.
                  J
                    (λchoice _.
                      Equal
                        (List a)
                        (match choice {
                          True ↦ Cons a h (filter a q (filter a r t));
                          False ↦ filter a q (filter a r t)
                        })
                        (Cons a h (filter a s t)))
                    (cong
                      (List a)
                      (List a)
                      (filter a q (filter a r t))
                      (filter a s t)
                      (λtail. Cons a h tail)
                      ih)
                    (sym Bool (q h) True kept);
            False ↦ λkept. λjoined. absurd (sym Bool True False joined)
          };
        False ↦
          match ds {
            True ↦ λkept. λjoined. absurd joined;
            False ↦
              λkept.
                λjoined.
                  J
                    (λchoice _.
                      Equal
                        (List a)
                        (match choice {
                          True ↦ Cons a h (filter a q (filter a r t));
                          False ↦ filter a q (filter a r t)
                        })
                        (filter a s t))
                    ih
                    (sym Bool (q h) False kept)
          }
      };
    False ↦
      match ds {
        True ↦ λkept. λjoined. absurd joined;
        False ↦ λkept. λjoined. ih
      }
  }

theorem window_filter_fuse
      (a : Type)
      (r : a → Bool)
      (q : a → Bool)
      (s : a → Bool)
      (joined : (x : a) → Equal Bool (bool_and (r x) (q x)) (s x))
      (xs : List a)
    : Equal (List a) (filter a q (filter a r xs)) (filter a s xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      window_filter_fuse_cons
        a
        r
        q
        s
        h
        t
        (window_filter_fuse a r q s joined t)
        (r h)
        (q h)
        (s h)
        Refl
        (joined h)
  }

theorem trace_window_split_cons
      (p : Type)
      (lo : Nat)
      (mid : Nat)
      (hi : Nat)
      (h : Event p)
      (t : List (Event p))
      (ordered : time_ordered p (Cons (Event p) h t))
      (lo_mid : Equal Bool (leq_nat lo mid) True)
      (mid_hi : Equal Bool (leq_nat mid hi) True)
      (ih : Equal
        (List (Event p))
        (trace_window p lo hi t)
        (list_append (Event p) (trace_window p lo mid t) (trace_window p mid hi t)))
      (below : Bool)
    : Equal Bool (lt_nat (event_time p h) mid) below
      → Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) h t))
        (list_append
          (Event p)
          (trace_window p lo mid (Cons (Event p) h t))
          (trace_window p mid hi (Cons (Event p) h t))) =
  match below {
    True ↦ λbelow_mid. trace_window_split_below p lo mid hi h t mid_hi ih below_mid;
    False ↦
      λnot_below.
        trace_window_split_above
          p
          lo
          mid
          hi
          h
          t
          ordered
          lo_mid
          ((proof false_reverse_leq for lt_nat) (event_time p h) mid not_below)
  }

theorem trace_window_split_below
      (p : Type)
      (lo : Nat)
      (mid : Nat)
      (hi : Nat)
      (h : Event p)
      (t : List (Event p))
      (mid_hi : Equal Bool (leq_nat mid hi) True)
      (ih : Equal
        (List (Event p))
        (trace_window p lo hi t)
        (list_append (Event p) (trace_window p lo mid t) (trace_window p mid hi t)))
    : Equal Bool (lt_nat (event_time p h) mid) True
      → Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) h t))
        (list_append
          (Event p)
          (trace_window p lo mid (Cons (Event p) h t))
          (trace_window p mid hi (Cons (Event p) h t))) =
  match h {
    MkEvent th x ↦
      λbelow_mid.
        trans
          (List (Event p))
          (trace_window p lo hi (Cons (Event p) (MkEvent p th x) t))
          (window_keep
            (Event p)
            (MkEvent p th x)
            (trace_window p lo hi t)
            (bool_and (leq_nat lo th) True))
          (list_append
            (Event p)
            (trace_window p lo mid (Cons (Event p) (MkEvent p th x) t))
            (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
          (cong
            Bool
            (List (Event p))
            (lt_nat th hi)
            True
            (λd.
              window_keep
                (Event p)
                (MkEvent p th x)
                (trace_window p lo hi t)
                (bool_and (leq_nat lo th) d))
            ((proof from_leq_suc for lt_nat)
              th
              hi
              ((proof trans for leq_nat)
                (Suc th)
                mid
                hi
                ((proof to_leq_suc for lt_nat) th mid below_mid)
                mid_hi)))
          (trans
            (List (Event p))
            (window_keep
              (Event p)
              (MkEvent p th x)
              (trace_window p lo hi t)
              (bool_and (leq_nat lo th) True))
            (list_append
              (Event p)
              (window_keep
                (Event p)
                (MkEvent p th x)
                (trace_window p lo mid t)
                (bool_and (leq_nat lo th) True))
              (trace_window p mid hi t))
            (list_append
              (Event p)
              (trace_window p lo mid (Cons (Event p) (MkEvent p th x) t))
              (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
            (window_keep_append
              (Event p)
              (MkEvent p th x)
              (trace_window p lo hi t)
              (trace_window p lo mid t)
              (trace_window p mid hi t)
              ih
              (bool_and (leq_nat lo th) True))
            (sym
              (List (Event p))
              (list_append
                (Event p)
                (trace_window p lo mid (Cons (Event p) (MkEvent p th x) t))
                (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
              (list_append
                (Event p)
                (window_keep
                  (Event p)
                  (MkEvent p th x)
                  (trace_window p lo mid t)
                  (bool_and (leq_nat lo th) True))
                (trace_window p mid hi t))
              (trans
                (List (Event p))
                (list_append
                  (Event p)
                  (trace_window p lo mid (Cons (Event p) (MkEvent p th x) t))
                  (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
                (list_append
                  (Event p)
                  (window_keep
                    (Event p)
                    (MkEvent p th x)
                    (trace_window p lo mid t)
                    (bool_and (leq_nat lo th) True))
                  (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
                (list_append
                  (Event p)
                  (window_keep
                    (Event p)
                    (MkEvent p th x)
                    (trace_window p lo mid t)
                    (bool_and (leq_nat lo th) True))
                  (trace_window p mid hi t))
                (cong
                  Bool
                  (List (Event p))
                  (lt_nat th mid)
                  True
                  (λd.
                    list_append
                      (Event p)
                      (window_keep
                        (Event p)
                        (MkEvent p th x)
                        (trace_window p lo mid t)
                        (bool_and (leq_nat lo th) d))
                      (trace_window p mid hi (Cons (Event p) (MkEvent p th x) t)))
                  below_mid)
                (cong
                  Bool
                  (List (Event p))
                  (leq_nat mid th)
                  False
                  (λd.
                    list_append
                      (Event p)
                      (window_keep
                        (Event p)
                        (MkEvent p th x)
                        (trace_window p lo mid t)
                        (bool_and (leq_nat lo th) True))
                      (window_keep
                        (Event p)
                        (MkEvent p th x)
                        (trace_window p mid hi t)
                        (bool_and d (lt_nat th hi))))
                  (lt_nat_reverse_leq_false th mid (leq_nat mid th) Refl below_mid)))))
  }

theorem trace_window_split_above
      (p : Type)
      (lo : Nat)
      (mid : Nat)
      (hi : Nat)
      (h : Event p)
      (t : List (Event p))
      (ordered : time_ordered p (Cons (Event p) h t))
      (lo_mid : Equal Bool (leq_nat lo mid) True)
      (mid_h : Equal Bool (leq_nat mid (event_time p h)) True)
    : Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) h t))
        (list_append
          (Event p)
          (trace_window p lo mid (Cons (Event p) h t))
          (trace_window p mid hi (Cons (Event p) h t))) =
  trans
    (List (Event p))
    (trace_window p lo hi (Cons (Event p) h t))
    (trace_window p mid hi (Cons (Event p) h t))
    (list_append
      (Event p)
      (trace_window p lo mid (Cons (Event p) h t))
      (trace_window p mid hi (Cons (Event p) h t)))
    (trace_window_raise p lo mid hi h t ordered lo_mid mid_h)
    (cong
      (List (Event p))
      (List (Event p))
      (Nil (Event p))
      (trace_window p lo mid (Cons (Event p) h t))
      (λz. list_append (Event p) z (trace_window p mid hi (Cons (Event p) h t)))
      (sym
        (List (Event p))
        (trace_window p lo mid (Cons (Event p) h t))
        (Nil (Event p))
        (trace_window_past p lo mid h t ordered mid_h)))

theorem trace_window_past
      (p : Type) (lo : Nat) (mid : Nat) (h : Event p) (t : List (Event p))
    : time_ordered p (Cons (Event p) h t)
      → Equal Bool (leq_nat mid (event_time p h)) True
      → Equal (List (Event p)) (trace_window p lo mid (Cons (Event p) h t)) (Nil (Event p)) =
  match t {
    Nil ↦
      λordered.
        λmid_h.
          cong
            Bool
            (List (Event p))
            (event_in_window p lo mid h)
            False
            (λd. window_keep (Event p) h (Nil (Event p)) d)
            (event_in_window_past p lo mid h mid_h);
    Cons h2 t2 ↦
      λordered.
        λmid_h.
          trans
            (List (Event p))
            (trace_window p lo mid (Cons (Event p) h (Cons (Event p) h2 t2)))
            (trace_window p lo mid (Cons (Event p) h2 t2))
            (Nil (Event p))
            (cong
              Bool
              (List (Event p))
              (event_in_window p lo mid h)
              False
              (λd. window_keep (Event p) h (trace_window p lo mid (Cons (Event p) h2 t2)) d)
              (event_in_window_past p lo mid h mid_h))
            (trace_window_past
              p
              lo
              mid
              h2
              t2
              (list_sorted_tail (Event p) (event_before p) h (Cons (Event p) h2 t2) ordered)
              ((proof trans for leq_nat)
                mid
                (event_time p h)
                (event_time p h2)
                mid_h
                (list_sorted_head (Event p) (event_before p) h (Cons (Event p) h2 t2) ordered)))
  }

theorem trace_window_raise
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (h : Event p) (t : List (Event p))
    : time_ordered p (Cons (Event p) h t)
      → Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid (event_time p h)) True
      → Equal
        (List (Event p))
        (trace_window p lo hi (Cons (Event p) h t))
        (trace_window p mid hi (Cons (Event p) h t)) =
  match t {
    Nil ↦
      λordered.
        λlo_mid.
          λmid_h.
            cong
              Bool
              (List (Event p))
              (event_in_window p lo hi h)
              (event_in_window p mid hi h)
              (λd. window_keep (Event p) h (Nil (Event p)) d)
              (event_in_window_raise p lo mid hi h lo_mid mid_h);
    Cons h2 t2 ↦
      λordered.
        λlo_mid.
          λmid_h.
            trans
              (List (Event p))
              (trace_window p lo hi (Cons (Event p) h (Cons (Event p) h2 t2)))
              (window_keep
                (Event p)
                h
                (trace_window p mid hi (Cons (Event p) h2 t2))
                (event_in_window p lo hi h))
              (trace_window p mid hi (Cons (Event p) h (Cons (Event p) h2 t2)))
              (cong
                (List (Event p))
                (List (Event p))
                (trace_window p lo hi (Cons (Event p) h2 t2))
                (trace_window p mid hi (Cons (Event p) h2 t2))
                (λrest. window_keep (Event p) h rest (event_in_window p lo hi h))
                (trace_window_raise
                  p
                  lo
                  mid
                  hi
                  h2
                  t2
                  (list_sorted_tail (Event p) (event_before p) h (Cons (Event p) h2 t2) ordered)
                  lo_mid
                  ((proof trans for leq_nat)
                    mid
                    (event_time p h)
                    (event_time p h2)
                    mid_h
                    (list_sorted_head
                      (Event p)
                      (event_before p)
                      h
                      (Cons (Event p) h2 t2)
                      ordered))))
              (cong
                Bool
                (List (Event p))
                (event_in_window p lo hi h)
                (event_in_window p mid hi h)
                (λd. window_keep (Event p) h (trace_window p mid hi (Cons (Event p) h2 t2)) d)
                (event_in_window_raise p lo mid hi h lo_mid mid_h))
  }

theorem event_in_window_past
      (p : Type) (lo : Nat) (mid : Nat) (e : Event p)
    : Equal Bool (leq_nat mid (event_time p e)) True
      → Equal Bool (event_in_window p lo mid e) False =
  match e {
    MkEvent t x ↦
      λmid_t.
        trans
          Bool
          (bool_and (leq_nat lo t) (lt_nat t mid))
          (bool_and (leq_nat lo t) False)
          False
          (cong
            Bool
            Bool
            (lt_nat t mid)
            False
            (λd. bool_and (leq_nat lo t) d)
            ((proof leq_not_lt for lt_nat) mid t mid_t))
          ((proof comm for bool_and) (leq_nat lo t) False)
  }

theorem event_in_window_raise
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (e : Event p)
    : Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid (event_time p e)) True
      → Equal Bool (event_in_window p lo hi e) (event_in_window p mid hi e) =
  match e {
    MkEvent t x ↦
      λlo_mid.
        λmid_t.
          trans
            Bool
            (bool_and (leq_nat lo t) (lt_nat t hi))
            (lt_nat t hi)
            (bool_and (leq_nat mid t) (lt_nat t hi))
            (cong
              Bool
              Bool
              (leq_nat lo t)
              True
              (λd. bool_and d (lt_nat t hi))
              ((proof trans for leq_nat) lo mid t lo_mid mid_t))
            (sym
              Bool
              (bool_and (leq_nat mid t) (lt_nat t hi))
              (lt_nat t hi)
              (cong Bool Bool (leq_nat mid t) True (λd. bool_and d (lt_nat t hi)) mid_t))
  }

theorem lt_nat_reverse_leq_false
      (a : Nat) (b : Nat) (decision : Bool)
    : Equal Bool (leq_nat b a) decision
      → Equal Bool (lt_nat a b) True
      → Equal Bool (leq_nat b a) False =
  match decision {
    True ↦
      λba.
        λab.
          absurd
            (trans
              Bool
              True
              (lt_nat a b)
              False
              (sym Bool (lt_nat a b) True ab)
              ((proof leq_not_lt for lt_nat) b a ba));
    False ↦ λba. λab. ba
  }

theorem window_keep_append
      (a : Type)
      (h : a)
      (whole : List a)
      (left : List a)
      (right : List a)
      (split : Equal (List a) whole (list_append a left right))
      (decision : Bool)
    : Equal
        (List a)
        (window_keep a h whole decision)
        (list_append a (window_keep a h left decision) right) =
  match decision {
    True ↦ cong (List a) (List a) whole (list_append a left right) (λrest. Cons a h rest) split;
    False ↦ split
  }
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

To split a half-open window, supply ordered-trace evidence and proofs of
`lo ≤ mid` and `mid ≤ hi`. The checked `trace_window_split` returns a
structural equality with the append of the two adjacent windows. An event
at `mid` belongs only to the right window; the
[closed split seed](../../../../conformance/stdlib/time-and-events/seed-window-adjacent-split.md)
shows both subwindows as nonempty lists.

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

Window composition combines two half-open predicates using the private
natural-order splits for `max` and `min`, then fuses the two filters over the
same input list. It needs neither ordering premises nor valid bounds. The
left empty-merge unit closes by reduction; the right unit exports the
existing checked induction rather than adding a second proof. The filter
fusion and natural-order split helpers remain private to this package.

Merge associativity holds as structural equality for arbitrary traces,
not just `time_ordered` inputs. The proof compares current heads by
`event_before` and reuses the now-public `trace_merge_nil_right` plus
three private lemmas: `trace_merge_cons_left`, `trace_merge_cons_right`
and `trace_head_after_decide`. Its seven new support theorems are private.
For the cases in which neither of two heads leads, natural-time totality
and transitivity determine the same first event under either grouping.
A timestamp tie is taken from the left input in both groupings, so
the final list retains the same stored events in the same order.

The adjacent-window proof inducts on the ordered trace and splits on whether
the head time is below the midpoint. A head below it is kept by the whole
window exactly when kept by the left subwindow and never by the right;
`window_keep_append` carries the induction hypothesis. At or above the
midpoint, ordering forces the left subwindow empty and the whole window to
match the right. The nine split helpers remain private, and the theorem
requires both bound-order premises.

Windowing distributes over appending arbitrary traces, including unordered
traces and inverted window bounds. The checked `trace_window_append` proof
instantiates the collections provider's public `filter::append` law with the
half-open event predicate; no additional helper or import is needed.

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
  — the carrier, half-open boundary, left-bias and fifteen checked law contracts.
- **Collections provider** —
  [Derived collections](../../Data/Collections/Derived.ken.md)
  — generic filtering, list membership, sorted bridges and counting.

## 7. Trust & derivation

1. **Public API.** `Event`, `MkEvent`, its two projections, the time
   comparison, ordering predicate, window predicate, `trace_window`,
   `trace_merge`, and the fifteen laws form the package surface.
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
   laws and remains private. Window composition reuses checked natural-order
   `max`/`min` bounds and structurally fuses generic list filters. Both merge
   units use the original merge operation; the right unit reuses its existing
   induction, now public under the same name. Merge associativity uses that
   public right unit and the three private head-selection lemmas, with seven
   new private support theorems; it needs no ordered-input premise. The
   adjacent-window split reuses the checked `list_append` provider and nine
   new private proof helpers under ordered-trace and bound-order premises.
   Window append reuses the public `filter::append` proof from Derived with
   the existing event predicate, without a new helper or import.
4. **Trusted-base delta.** Zero: all fifteen exported laws have checked proof
   bodies, and this package adds no `Axiom` or primitive. Imported logical,
   numeric and collection providers retain their inherited trust.
5. **Validation.** The
   [trace seed](../../../../conformance/stdlib/time-and-events/seed-event-trace.md),
   [window-membership seed](../../../../conformance/stdlib/time-and-events/seed-window-membership.md),
   [distribution seed](../../../../conformance/stdlib/time-and-events/seed-window-merge-distribution.md),
   [merge-membership seed](../../../../conformance/stdlib/time-and-events/seed-merge-membership.md),
   [composition seed](../../../../conformance/stdlib/time-and-events/seed-window-composition.md),
   [associativity seed](../../../../conformance/stdlib/time-and-events/seed-merge-associativity.md),
   [adjacent-split seed](../../../../conformance/stdlib/time-and-events/seed-window-adjacent-split.md)
   and [window-append seed](../../../../conformance/stdlib/time-and-events/seed-window-append-distribution.md)
   supply independent closed values and refused claims; these clients
   complement, not replace, the generic proofs. Package tests run the
   merge-membership and composition seeds, including their pinned refusals.
   The adjacent-split package test root-loads the checked generic law,
   accepts the complete seed and refuses its isolated swapped claim.
   The `cat_time_window_append` package test checks the §8 law and the
   complete accepted window-append seed with both isolated swapped refusals.
   These tests are not official conformance-runner execution or CI.
