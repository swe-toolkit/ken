# `Argmin` — earliest minimum- and maximum-cost candidates

`argmin` and `argmax` select existing candidates with the least and greatest
natural-number costs, respectively, retaining the earliest candidate on a
tie. `argmin_option` and `argmax_option` select from possibly empty lists,
returning `None` for `Nil`. Both nonempty choices are unchanged by a cost
transform that preserves and reflects every strict natural-cost comparison.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

A nonempty finite input always has least- and greatest-cost candidates
without requiring an equality operation on its values. Strictly comparing
each tail candidate against the current head retains the head whenever two
costs are equal. The existing `NonEmpty` carrier supplies the head and ordered
tail. On a possibly empty `List`, each Option selector returns no value on
`Nil` and delegates `Cons` to its corresponding nonempty selector.

## Definition

The minimum selector and its ten laws lead, followed by its Option selector
and four laws. The maximum selector and its eight laws follow, then its Option
selector and four laws. Private shared selection helpers, inductive proofs and
Option proof helpers close the definition. Two generic Nat-order facts come
from their canonical owner, `Data.Numeric.Nat.Order`.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)

import Core.Function.Combinators (comp)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived (list_append, list_elem, list_elem_head, list_elem_later)

import Data.Collections.NonEmpty
  (NonEmpty,
    nonempty_cons,
    nonempty_singleton,
    nonempty_head,
    nonempty_tail,
    nonempty_to_list,
    nonempty_to_list_shape,
    nonempty_append)

pub fn argmin (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a =
  argmin_from a cost (nonempty_head a xs) (nonempty_tail a xs)

pub theorem argmin_member
      (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : list_elem a (argmin a cost xs) (nonempty_to_list a xs) =
  J
    (λl _.
      list_elem a (argmin a cost xs) l → list_elem a (argmin a cost xs) (nonempty_to_list a xs))
    (λmember. member)
    (nonempty_to_list_shape a xs)
    (argmin_from_member a cost (nonempty_head a xs) (nonempty_tail a xs))

pub theorem argmin_least
      (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : (y : a)
      → list_elem a y (nonempty_to_list a xs)
      → IsTrue (leq_nat (cost (argmin a cost xs)) (cost y)) =
  λy.
    λmember.
      argmin_from_least
        a
        cost
        (nonempty_head a xs)
        (nonempty_tail a xs)
        y
        (J (λl _. list_elem a y l) member (nonempty_to_list_shape a xs))

pub theorem argmin_no_cheaper
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (y : a)
    : list_elem a y (nonempty_to_list a xs)
      → Equal Bool (lt_nat (cost y) (cost (argmin a cost xs))) False =
  λmember.
    (proof leq_not_lt for lt_nat)
      (cost (argmin a cost xs))
      (cost y)
      (argmin_least a cost xs y member)

pub theorem argmin_head_on_tie
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : ((y : a) → list_elem a y rest → IsTrue (leq_nat (cost x) (cost y)))
      → Equal a (argmin a cost (nonempty_cons a x rest)) x =
  match rest {
    Nil ↦ λbound. Refl;
    Cons z zs ↦
      λbound.
        choose_no
          a
          (lt_nat (cost (argmin_from a cost z zs)) (cost x))
          (argmin_from a cost z zs)
          x
          ((proof leq_not_lt for lt_nat)
            (cost x)
            (cost (argmin_from a cost z zs))
            (bound (argmin_from a cost z zs) (argmin_from_member a cost z zs)))
  }

pub theorem argmin_singleton
      (a : Type) (cost : a → Nat) (x : a)
    : Equal a (argmin a cost (nonempty_singleton a x)) x =
  Refl

pub theorem argmin_keeps_head
      (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x)) False
      → Equal a (argmin a cost (nonempty_cons a x (Cons a y ys))) x =
  choose_no a (lt_nat (cost (argmin_from a cost y ys)) (cost x)) (argmin_from a cost y ys) x

pub theorem argmin_takes_tail
      (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost x)) True
      → Equal a
        (argmin a cost (nonempty_cons a x (Cons a y ys)))
        (argmin a cost (nonempty_cons a y ys)) =
  choose_yes a (lt_nat (cost (argmin_from a cost y ys)) (cost x)) (argmin_from a cost y ys) x

pub theorem argmin_cost_order_invariant
      (a : Type)
      (cost : a → Nat)
      (f : Nat → Nat)
      (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
      (xs : NonEmpty a)
    : Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a cost xs) =
  select_from_cost_order a cost lt_nat f same_order (nonempty_head a xs) (nonempty_tail a xs)

pub theorem argmin_append_keeps_left
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) False
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost xs) =
  λh.
    trans
      a
      (argmin a cost (nonempty_append a xs ys))
      (choose
        a
        (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
        (argmin a cost ys)
        (argmin a cost xs))
      (argmin a cost xs)
      (argmin_append_choice a cost xs ys)
      (choose_no
        a
        (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
        (argmin a cost ys)
        (argmin a cost xs)
        h)

pub theorem argmin_append_takes_right
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal Bool (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs))) True
      → Equal a (argmin a cost (nonempty_append a xs ys)) (argmin a cost ys) =
  λh.
    trans
      a
      (argmin a cost (nonempty_append a xs ys))
      (choose
        a
        (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
        (argmin a cost ys)
        (argmin a cost xs))
      (argmin a cost ys)
      (argmin_append_choice a cost xs ys)
      (choose_yes
        a
        (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
        (argmin a cost ys)
        (argmin a cost xs)
        h)

pub fn argmin_option (a : Type) (cost : a → Nat) (xs : List a) : Option a =
  match xs {
    Nil ↦ None a;
    Cons x rest ↦ Some a (argmin a cost (nonempty_cons a x rest))
  }

pub theorem argmin_option_nil
      (a : Type) (cost : a → Nat)
    : Equal (Option a) (argmin_option a cost (Nil a)) (None a) =
  Proved

pub theorem argmin_option_cons
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : Equal
        (Option a)
        (argmin_option a cost (Cons a x rest))
        (Some a (argmin a cost (nonempty_cons a x rest))) =
  Refl

pub theorem argmin_option_member
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmin_option a cost xs) (Some a y) → list_elem a y xs =
  λfound. J (λr _. option_member a xs r) (argmin_option_member_base a cost xs) found

pub theorem argmin_option_least
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmin_option a cost xs) (Some a y)
      → (z : a)
      → list_elem a z xs
      → IsTrue (leq_nat (cost y) (cost z)) =
  λfound. J (λr _. option_least a cost xs r) (argmin_option_least_base a cost xs) found

pub fn argmax (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a =
  argmax_from a cost (nonempty_head a xs) (nonempty_tail a xs)

pub theorem argmax_member
      (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : list_elem a (argmax a cost xs) (nonempty_to_list a xs) =
  J
    (λl _.
      list_elem a (argmax a cost xs) l → list_elem a (argmax a cost xs) (nonempty_to_list a xs))
    (λmember. member)
    (nonempty_to_list_shape a xs)
    (select_from_member a cost costlier (nonempty_head a xs) (nonempty_tail a xs))

pub theorem argmax_greatest
      (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : (y : a)
      → list_elem a y (nonempty_to_list a xs)
      → IsTrue (leq_nat (cost y) (cost (argmax a cost xs))) =
  λy.
    λmember.
      argmax_from_greatest
        a
        cost
        (nonempty_head a xs)
        (nonempty_tail a xs)
        y
        (J (λl _. list_elem a y l) member (nonempty_to_list_shape a xs))

pub theorem argmax_no_costlier
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (y : a)
    : list_elem a y (nonempty_to_list a xs)
      → Equal Bool (lt_nat (cost (argmax a cost xs)) (cost y)) False =
  λmember.
    (proof leq_not_lt for lt_nat)
      (cost y)
      (cost (argmax a cost xs))
      (argmax_greatest a cost xs y member)

pub theorem argmax_head_on_tie
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : ((y : a) → list_elem a y rest → IsTrue (leq_nat (cost y) (cost x)))
      → Equal a (argmax a cost (nonempty_cons a x rest)) x =
  match rest {
    Nil ↦ λbound. Refl;
    Cons z zs ↦
      λbound.
        choose_no
          a
          (costlier (cost (argmax_from a cost z zs)) (cost x))
          (argmax_from a cost z zs)
          x
          ((proof leq_not_lt for lt_nat)
            (cost (argmax_from a cost z zs))
            (cost x)
            (bound (argmax_from a cost z zs) (select_from_member a cost costlier z zs)))
  }

pub theorem argmax_singleton
      (a : Type) (cost : a → Nat) (x : a)
    : Equal a (argmax a cost (nonempty_singleton a x)) x =
  Refl

pub theorem argmax_keeps_head
      (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) False
      → Equal a (argmax a cost (nonempty_cons a x (Cons a y ys))) x =
  choose_no a (costlier (cost (argmax_from a cost y ys)) (cost x)) (argmax_from a cost y ys) x

pub theorem argmax_takes_tail
      (a : Type) (cost : a → Nat) (x : a) (y : a) (ys : List a)
    : Equal Bool (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) True
      → Equal a
        (argmax a cost (nonempty_cons a x (Cons a y ys)))
        (argmax a cost (nonempty_cons a y ys)) =
  choose_yes a (costlier (cost (argmax_from a cost y ys)) (cost x)) (argmax_from a cost y ys) x

pub theorem argmax_cost_order_invariant
      (a : Type)
      (cost : a → Nat)
      (f : Nat → Nat)
      (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
      (xs : NonEmpty a)
    : Equal a (argmax a (comp a Nat Nat f cost) xs) (argmax a cost xs) =
  select_from_cost_order
    a
    cost
    costlier
    f
    (λi. λj. same_order j i)
    (nonempty_head a xs)
    (nonempty_tail a xs)

pub fn argmax_option (a : Type) (cost : a → Nat) (xs : List a) : Option a =
  match xs {
    Nil ↦ None a;
    Cons x rest ↦ Some a (argmax a cost (nonempty_cons a x rest))
  }

pub theorem argmax_option_nil
      (a : Type) (cost : a → Nat)
    : Equal (Option a) (argmax_option a cost (Nil a)) (None a) =
  Proved

pub theorem argmax_option_cons
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : Equal
        (Option a)
        (argmax_option a cost (Cons a x rest))
        (Some a (argmax a cost (nonempty_cons a x rest))) =
  Refl

pub theorem argmax_option_member
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmax_option a cost xs) (Some a y) → list_elem a y xs =
  λfound. J (λr _. option_member a xs r) (argmax_option_member_base a cost xs) found

pub theorem argmax_option_greatest
      (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    : Equal (Option a) (argmax_option a cost xs) (Some a y)
      → (z : a)
      → list_elem a z xs
      → IsTrue (leq_nat (cost z) (cost y)) =
  λfound. J (λr _. option_greatest a cost xs r) (argmax_option_greatest_base a cost xs) found

fn argmin_from (a : Type) (cost : a → Nat) (x : a) (rest : List a) : a =
  select_from a cost lt_nat x rest

fn argmax_from (a : Type) (cost : a → Nat) (x : a) (rest : List a) : a =
  select_from a cost costlier x rest

fn costlier (challenger : Nat) (incumbent : Nat) : Bool = lt_nat incumbent challenger

fn select_from
      (a : Type) (cost : a → Nat) (better : Nat → Nat → Bool) (x : a) (rest : List a)
    : a =
  match rest {
    Nil ↦ x;
    Cons y ys ↦
      let tail_choice =
        select_from a cost better y ys
      in
        choose a (better (cost tail_choice) (cost x)) tail_choice x
  }

fn choose (a : Type) (b : Bool) (yes : a) (no : a) : a =
  match b {
    True ↦ yes;
    False ↦ no
  }

theorem argmin_from_member
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : list_elem a (argmin_from a cost x rest) (Cons a x rest) =
  select_from_member a cost lt_nat x rest

theorem select_from_member
      (a : Type) (cost : a → Nat) (better : Nat → Nat → Bool) (x : a) (rest : List a)
    : list_elem a (select_from a cost better x rest) (Cons a x rest) =
  match rest {
    Nil ↦ list_elem_head a x (Nil a);
    Cons y ys ↦
      choose_elem
        a
        (better (cost (select_from a cost better y ys)) (cost x))
        (select_from a cost better y ys)
        x
        (Cons a y ys)
        (select_from_member a cost better y ys)
  }

theorem argmin_from_least
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : (y : a)
      → list_elem a y (Cons a x rest)
      → IsTrue (leq_nat (cost (argmin_from a cost x rest)) (cost y)) =
  match rest {
    Nil ↦
      λy.
        λmember.
          elim_trunc
            (IsTrue (leq_nat (cost x) (cost y)))
            (λcase.
              match case {
                Inl same ↦
                  J
                    (λv _. IsTrue (leq_nat (cost v) (cost y)))
                    ((proof refl for leq_nat) (cost y))
                    same;
                Inr later ↦ absurd later
              })
            member;
    Cons z zs ↦
      λy.
        λmember.
          elim_trunc
            (IsTrue (leq_nat (cost (argmin_from a cost x (Cons a z zs))) (cost y)))
            (λcase.
              match case {
                Inl same ↦
                  J
                    (λv _.
                      IsTrue (leq_nat (cost (argmin_from a cost x (Cons a z zs))) (cost v))
                      → IsTrue (leq_nat (cost (argmin_from a cost x (Cons a z zs))) (cost y)))
                    (λh. h)
                    same
                    (choose_le_no
                      a
                      cost
                      (argmin_from a cost z zs)
                      x
                      (lt_nat (cost (argmin_from a cost z zs)) (cost x))
                      Refl);
                Inr later ↦
                  (proof trans for leq_nat)
                    (cost (argmin_from a cost x (Cons a z zs)))
                    (cost (argmin_from a cost z zs))
                    (cost y)
                    (choose_le_yes
                      a
                      cost
                      (argmin_from a cost z zs)
                      x
                      (lt_nat (cost (argmin_from a cost z zs)) (cost x))
                      Refl)
                    (argmin_from_least a cost z zs y later)
              })
            member
  }

theorem choose_elem
      (a : Type) (b : Bool) (yes : a) (no : a) (later : List a)
    : list_elem a yes later → list_elem a (choose a b yes no) (Cons a no later) =
  match b {
    True ↦ λmember. list_elem_later a yes no later member;
    False ↦ λmember. list_elem_head a no later
  }

theorem choose_no
      (a : Type) (b : Bool) (yes : a) (no : a)
    : Equal Bool b False → Equal a (choose a b yes no) no =
  match b {
    True ↦ λh. absurd h;
    False ↦ λh. Refl
  }

theorem choose_yes
      (a : Type) (b : Bool) (yes : a) (no : a)
    : Equal Bool b True → Equal a (choose a b yes no) yes =
  match b {
    True ↦ λh. Refl;
    False ↦ λh. absurd h
  }

fn argmin_view (a : Type) (cost : a → Nat) (fallback : a) (l : List a) : a =
  match l {
    Nil ↦ fallback;
    Cons x rest ↦ argmin_from a cost x rest
  }

theorem argmin_via_view
      (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    : Equal a
        (argmin a cost xs)
        (argmin_view a cost (nonempty_head a xs) (nonempty_to_list a xs)) =
  J
    (λl _. Equal a (argmin a cost xs) (argmin_view a cost (nonempty_head a xs) l))
    Refl
    (sym
      (List a)
      (nonempty_to_list a xs)
      (Cons a (nonempty_head a xs) (nonempty_tail a xs))
      (nonempty_to_list_shape a xs))

theorem argmin_append_choice
      (a : Type) (cost : a → Nat) (xs : NonEmpty a) (ys : NonEmpty a)
    : Equal a
        (argmin a cost (nonempty_append a xs ys))
        (choose
          a
          (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
          (argmin a cost ys)
          (argmin a cost xs)) =
  let d : a =
    nonempty_head a (nonempty_append a xs ys)
  in
    trans
      a
      (argmin a cost (nonempty_append a xs ys))
      (argmin_view a cost d (nonempty_to_list a (nonempty_append a xs ys)))
      (choose
        a
        (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
        (argmin a cost ys)
        (argmin a cost xs))
      (argmin_via_view a cost (nonempty_append a xs ys))
      (trans
        a
        (argmin_view a cost d (nonempty_to_list a (nonempty_append a xs ys)))
        (argmin_view a cost d (list_append a (nonempty_to_list a xs) (nonempty_to_list a ys)))
        (choose
          a
          (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
          (argmin a cost ys)
          (argmin a cost xs))
        (cong
          (List a)
          a
          (nonempty_to_list a (nonempty_append a xs ys))
          (list_append a (nonempty_to_list a xs) (nonempty_to_list a ys))
          (λl. argmin_view a cost d l)
          ((proof list_view for nonempty_append) a xs ys))
        (trans
          a
          (argmin_view a cost d (list_append a (nonempty_to_list a xs) (nonempty_to_list a ys)))
          (argmin_view
            a
            cost
            d
            (list_append
              a
              (Cons a (nonempty_head a xs) (nonempty_tail a xs))
              (Cons a (nonempty_head a ys) (nonempty_tail a ys))))
          (choose
            a
            (lt_nat (cost (argmin a cost ys)) (cost (argmin a cost xs)))
            (argmin a cost ys)
            (argmin a cost xs))
          (J
            (λl _.
              Equal
                a
                (argmin_view
                  a
                  cost
                  d
                  (list_append a (nonempty_to_list a xs) (nonempty_to_list a ys)))
                (argmin_view
                  a
                  cost
                  d
                  (list_append a (Cons a (nonempty_head a xs) (nonempty_tail a xs)) l)))
            (cong
              (List a)
              a
              (nonempty_to_list a xs)
              (Cons a (nonempty_head a xs) (nonempty_tail a xs))
              (λl. argmin_view a cost d (list_append a l (nonempty_to_list a ys)))
              (nonempty_to_list_shape a xs))
            (nonempty_to_list_shape a ys))
          (argmin_from_append
            a
            cost
            (nonempty_head a xs)
            (nonempty_tail a xs)
            (nonempty_head a ys)
            (nonempty_tail a ys))))

theorem argmin_from_append
      (a : Type) (cost : a → Nat) (x : a) (rest : List a) (y : a) (more : List a)
    : Equal a
        (argmin_from a cost x (list_append a rest (Cons a y more)))
        (choose
          a
          (lt_nat (cost (argmin_from a cost y more)) (cost (argmin_from a cost x rest)))
          (argmin_from a cost y more)
          (argmin_from a cost x rest)) =
  match rest {
    Nil ↦ Refl;
    Cons z zs ↦
      J
        (λt _.
          Equal
            a
            (choose a (lt_nat (cost t) (cost x)) t x)
            (choose
              a
              (lt_nat
                (cost (argmin_from a cost y more))
                (cost (argmin_from a cost x (Cons a z zs))))
              (argmin_from a cost y more)
              (argmin_from a cost x (Cons a z zs))))
        (choose_earliest_assoc
          a
          cost
          x
          (argmin_from a cost z zs)
          (argmin_from a cost y more)
          (lt_nat (cost (argmin_from a cost z zs)) (cost x))
          (lt_nat (cost (argmin_from a cost y more)) (cost (argmin_from a cost z zs)))
          Refl
          Refl)
        (sym
          a
          (argmin_from a cost z (list_append a zs (Cons a y more)))
          (choose
            a
            (lt_nat (cost (argmin_from a cost y more)) (cost (argmin_from a cost z zs)))
            (argmin_from a cost y more)
            (argmin_from a cost z zs))
          (argmin_from_append a cost z zs y more))
  }

theorem choose_earliest_assoc
      (a : Type) (cost : a → Nat) (x : a) (l : a) (r : a) (b1 : Bool) (b2 : Bool)
    : Equal Bool (lt_nat (cost l) (cost x)) b1
      → Equal Bool (lt_nat (cost r) (cost l)) b2
      → Equal a
        (choose a (lt_nat (cost (choose a b2 r l)) (cost x)) (choose a b2 r l) x)
        (choose a (lt_nat (cost r) (cost (choose a b1 l x))) r (choose a b1 l x)) =
  match b1 {
    True ↦
      match b2 {
        True ↦
          λh1.
            λh2.
              trans
                a
                (choose a (lt_nat (cost r) (cost x)) r x)
                r
                (choose a (lt_nat (cost r) (cost l)) r l)
                (choose_yes
                  a
                  (lt_nat (cost r) (cost x))
                  r
                  x
                  ((proof trans for lt_nat) (cost r) (cost l) (cost x) h2 h1))
                (sym
                  a
                  (choose a (lt_nat (cost r) (cost l)) r l)
                  r
                  (choose_yes a (lt_nat (cost r) (cost l)) r l h2));
        False ↦
          λh1.
            λh2.
              trans
                a
                (choose a (lt_nat (cost l) (cost x)) l x)
                l
                (choose a (lt_nat (cost r) (cost l)) r l)
                (choose_yes a (lt_nat (cost l) (cost x)) l x h1)
                (sym
                  a
                  (choose a (lt_nat (cost r) (cost l)) r l)
                  l
                  (choose_no a (lt_nat (cost r) (cost l)) r l h2))
      };
    False ↦
      match b2 {
        True ↦ λh1. λh2. Refl;
        False ↦
          λh1.
            λh2.
              trans
                a
                (choose a (lt_nat (cost l) (cost x)) l x)
                x
                (choose a (lt_nat (cost r) (cost x)) r x)
                (choose_no a (lt_nat (cost l) (cost x)) l x h1)
                (sym
                  a
                  (choose a (lt_nat (cost r) (cost x)) r x)
                  x
                  (choose_no
                    a
                    (lt_nat (cost r) (cost x))
                    r
                    x
                    ((proof leq_not_lt for lt_nat)
                      (cost x)
                      (cost r)
                      ((proof trans for leq_nat)
                        (cost x)
                        (cost l)
                        (cost r)
                        ((proof false_reverse_leq for lt_nat) (cost l) (cost x) h1)
                        ((proof false_reverse_leq for lt_nat) (cost r) (cost l) h2)))))
      }
  }

theorem select_from_cost_order
      (a : Type)
      (cost : a → Nat)
      (better : Nat → Nat → Bool)
      (f : Nat → Nat)
      (same_better : (i : Nat) → (j : Nat) → Equal Bool (better (f i) (f j)) (better i j))
      (x : a)
      (rest : List a)
    : Equal a
        (select_from a (comp a Nat Nat f cost) better x rest)
        (select_from a cost better x rest) =
  match rest {
    Nil ↦ Refl;
    Cons y ys ↦
      J
        (λr _.
          Equal
            a
            (choose
              a
              (better
                (f (cost (select_from a (comp a Nat Nat f cost) better y ys)))
                (f (cost x)))
              (select_from a (comp a Nat Nat f cost) better y ys)
              x)
            (choose a (better (cost r) (cost x)) r x))
        (choose_cost_order
          a
          cost
          better
          f
          same_better
          (select_from a (comp a Nat Nat f cost) better y ys)
          x)
        (select_from_cost_order a cost better f same_better y ys)
  }

theorem choose_cost_order
      (a : Type)
      (cost : a → Nat)
      (better : Nat → Nat → Bool)
      (f : Nat → Nat)
      (same_better : (i : Nat) → (j : Nat) → Equal Bool (better (f i) (f j)) (better i j))
      (yes : a)
      (no : a)
    : Equal a
        (choose a (better (f (cost yes)) (f (cost no))) yes no)
        (choose a (better (cost yes) (cost no)) yes no) =
  J
    (λb _. Equal a (choose a (better (f (cost yes)) (f (cost no))) yes no) (choose a b yes no))
    Refl
    (same_better (cost yes) (cost no))

theorem choose_le_no
      (a : Type) (cost : a → Nat) (yes : a) (no : a) (b : Bool)
    : Equal Bool (lt_nat (cost yes) (cost no)) b
      → IsTrue (leq_nat (cost (choose a b yes no)) (cost no)) =
  match b {
    True ↦ λh. (proof true_leq for lt_nat) (cost yes) (cost no) h;
    False ↦ λh. (proof refl for leq_nat) (cost no)
  }

theorem choose_le_yes
      (a : Type) (cost : a → Nat) (yes : a) (no : a) (b : Bool)
    : Equal Bool (lt_nat (cost yes) (cost no)) b
      → IsTrue (leq_nat (cost (choose a b yes no)) (cost yes)) =
  match b {
    True ↦ λh. (proof refl for leq_nat) (cost yes);
    False ↦ λh. (proof false_reverse_leq for lt_nat) (cost yes) (cost no) h
  }

theorem argmax_from_greatest
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : (y : a)
      → list_elem a y (Cons a x rest)
      → IsTrue (leq_nat (cost y) (cost (argmax_from a cost x rest))) =
  match rest {
    Nil ↦
      λy.
        λmember.
          elim_trunc
            (IsTrue (leq_nat (cost y) (cost x)))
            (λcase.
              match case {
                Inl same ↦
                  J
                    (λv _. IsTrue (leq_nat (cost y) (cost v)))
                    ((proof refl for leq_nat) (cost y))
                    same;
                Inr later ↦ absurd later
              })
            member;
    Cons z zs ↦
      λy.
        λmember.
          elim_trunc
            (IsTrue (leq_nat (cost y) (cost (argmax_from a cost x (Cons a z zs)))))
            (λcase.
              match case {
                Inl same ↦
                  J
                    (λv _.
                      IsTrue (leq_nat (cost v) (cost (argmax_from a cost x (Cons a z zs))))
                      → IsTrue (leq_nat (cost y) (cost (argmax_from a cost x (Cons a z zs)))))
                    (λh. h)
                    same
                    (choose_ge_no
                      a
                      cost
                      (argmax_from a cost z zs)
                      x
                      (costlier (cost (argmax_from a cost z zs)) (cost x))
                      Refl);
                Inr later ↦
                  (proof trans for leq_nat)
                    (cost y)
                    (cost (argmax_from a cost z zs))
                    (cost (argmax_from a cost x (Cons a z zs)))
                    (argmax_from_greatest a cost z zs y later)
                    (choose_ge_yes
                      a
                      cost
                      (argmax_from a cost z zs)
                      x
                      (costlier (cost (argmax_from a cost z zs)) (cost x))
                      Refl)
              })
            member
  }

theorem choose_ge_no
      (a : Type) (cost : a → Nat) (yes : a) (no : a) (b : Bool)
    : Equal Bool (lt_nat (cost no) (cost yes)) b
      → IsTrue (leq_nat (cost no) (cost (choose a b yes no))) =
  match b {
    True ↦ λh. (proof true_leq for lt_nat) (cost no) (cost yes) h;
    False ↦ λh. (proof refl for leq_nat) (cost no)
  }

theorem choose_ge_yes
      (a : Type) (cost : a → Nat) (yes : a) (no : a) (b : Bool)
    : Equal Bool (lt_nat (cost no) (cost yes)) b
      → IsTrue (leq_nat (cost yes) (cost (choose a b yes no))) =
  match b {
    True ↦ λh. (proof refl for leq_nat) (cost yes);
    False ↦ λh. (proof false_reverse_leq for lt_nat) (cost no) (cost yes) h
  }

fn option_member (a : Type) (xs : List a) (outcome : Option a) : Prop =
  match outcome {
    None ↦ Top;
    Some y ↦ list_elem a y xs
  }

theorem argmin_option_member_base
      (a : Type) (cost : a → Nat) (xs : List a)
    : option_member a xs (argmin_option a cost xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦ argmin_member a cost (nonempty_cons a x rest)
  }

fn option_least (a : Type) (cost : a → Nat) (xs : List a) (outcome : Option a) : Prop =
  match outcome {
    None ↦ Top;
    Some y ↦ (z : a) → list_elem a z xs → IsTrue (leq_nat (cost y) (cost z))
  }

theorem argmin_option_least_base
      (a : Type) (cost : a → Nat) (xs : List a)
    : option_least a cost xs (argmin_option a cost xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦ argmin_least a cost (nonempty_cons a x rest)
  }

theorem argmax_option_member_base
      (a : Type) (cost : a → Nat) (xs : List a)
    : option_member a xs (argmax_option a cost xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦ argmax_member a cost (nonempty_cons a x rest)
  }

fn option_greatest (a : Type) (cost : a → Nat) (xs : List a) (outcome : Option a) : Prop =
  match outcome {
    None ↦ Top;
    Some y ↦ (z : a) → list_elem a z xs → IsTrue (leq_nat (cost z) (cost y))
  }

theorem argmax_option_greatest_base
      (a : Type) (cost : a → Nat) (xs : List a)
    : option_greatest a cost xs (argmax_option a cost xs) =
  match xs {
    Nil ↦ Proved;
    Cons x rest ↦ argmax_greatest a cost (nonempty_cons a x rest)
  }
```

## Using it

Call `argmin` or `argmax` with the value type, a natural-number cost
function and an existing `NonEmpty` value. Construct inputs with
`nonempty_singleton` or `nonempty_cons`; use `nonempty_to_list` to reason
about the full ordered input. No `DecEq` or order on the candidate type is
required. A singleton returns its sole value; a later candidate replaces the
head only when its cost is strictly lower for `argmin`, or strictly higher for
`argmax`. For a possibly empty `List`, call `argmin_option` for least cost or
`argmax_option` for greatest cost. Each returns `None` for `Nil`; on `Some y`,
its corollaries establish membership and the corresponding cost bound on the
original list. For a total cost transform preserving every strict comparison
in both directions, `argmin_cost_order_invariant` and
`argmax_cost_order_invariant` identify the same input candidate before and
after transforming its costs, including the earlier winner of a tie. For two
non-empty parts, `argmin_append_keeps_left` retains the left choice when the
right minimum is not strictly cheaper, including ties;
`argmin_append_takes_right` selects the right choice when it is strictly
cheaper.

## Laws & proofs

`argmin_member` transports the checked membership of the private head/tail
selection to the public list view by `nonempty_to_list_shape`.
`argmin_least` proves a cost bound for every member by eliminating truncated
`list_elem` only into an Ω-valued proposition. `argmin_no_cheaper` uses the
canonical Nat-order bridge to rule out each strictly cheaper member.

`argmin_head_on_tie` keeps the head when its cost bounds every tail member;
`argmin_singleton`, `argmin_keeps_head`, and `argmin_takes_tail` state the
base case and both outcomes of strict comparison. The private selection
proofs distinguish those outcomes by binding the comparison Boolean and its
corresponding equality rather than assuming that an indirectly converted
expression is refined by a match. `argmin_cost_order_invariant` relates two
costs: when `f` gives every strict Nat comparison its original Boolean, each
step of the private selection compares the same way under
`comp a Nat Nat f cost` as under `cost`, so the same candidate is chosen, ties
included. The private `select_from_cost_order` inducts over the tail and
transports the head comparison with `J`; the premise cannot be dropped, since
a cost transform that merges two costs turns them into a tie that keeps the
earlier candidate. The generic strict/non-strict order bridges are public
proofs attached to `lt_nat` in `Data.Numeric.Nat.Order`, not
additional optimization laws or duplicated local recursion.

`argmin_append_keeps_left` and `argmin_append_takes_right` split on the
strict comparison of the right part's minimum cost against the left part's.
The private `argmin_view` transports `argmin` through `nonempty_to_list_shape`,
then `argmin_append_choice` uses the public `nonempty_append` list-view proof
to combine the two parts. `argmin_from_append` inducts over the left tail;
`choose_earliest_assoc` preserves the earliest tied choice across the split.
Both public laws finish by selecting the corresponding branch of the private
`choose`. No constructor of `NonEmpty` is exposed in these proofs.

`argmin_option_nil` and `argmin_option_cons` reduce the selector on both list
constructors. The private `option_member` and `option_least` predicates assign
`Top` to a missing result and the corresponding list proposition to `Some`.
Their private base proofs use the existing `argmin_member` and `argmin_least`
in the `Cons` case. `J` transports these proofs along the observed result
equality to give the public membership and least-cost corollaries.

`argmax_member` transports the shared private selection-membership proof to
the public list view. `argmax_greatest` inducts over the ordered tail and
eliminates truncated membership only into `IsTrue`, bounding every member's
cost by the selected cost. `argmax_no_costlier` uses the checked Nat-order
bridge. `argmax_head_on_tie` uses the greatest bound and the strict-order
bridge to keep the head, even when a tail candidate ties it. The singleton
and both strict-comparison branch laws fix the choice on every input.
`argmax_cost_order_invariant` uses the same public strict-order comparison
premise as its minimum counterpart, swapping the indices when passing it to
the private `costlier` comparison. The shared `select_from_cost_order`
induction and `choose_cost_order` transport each choice across the cost
transform for either selector. Equal original costs remain tied, including a
maximum tied away from the head; flattening distinct costs is excluded by the
premise.

`argmax_option_nil` and `argmax_option_cons` reduce the maximum selector on
both list constructors. The existing private `option_member` assigns `Top`
to `None` and membership to `Some`. New private `option_greatest` similarly
assigns `Top` to `None` and a greatest-cost bound to `Some`. Selector-specific
base proofs use `argmax_member` and `argmax_greatest` for `Cons`; `J` carries
those proofs along the observed `Some` equality to the public corollaries.

## Design notes

`argmin_from` and `argmax_from` share a private `select_from` fold over the
ordered tail. The former compares challenger with incumbent; the latter's
`costlier` reverses those cost arguments. `choose` keeps the incumbent on a
false comparison, including ties. The shared membership proof depends only
on selection from the two candidates, not on cost direction. Membership is
propositionally truncated: it certifies that the chosen value was present but
is not an index or a Boolean equality on arbitrary candidate values. Neither
nonempty selector promises a complexity bound. The cost-order proof helpers
are private and parameterized by the existing strict comparison, so the
minimum and maximum laws share the induction without changing either
selector's tie behavior or the minimum law's public type. Each Option
selector handles an empty list by returning `None`, without another selection
algorithm.

## References

- **Arg max — Wikipedia** — https://en.wikipedia.org/wiki/Arg_max —
  orientation on maximum-argument selection and ties among maximizers.
- **Ken optimization contract** — `spec/50-stdlib/72-optimization.md` §§1, 3 —
  the nonempty selectors, cost-order laws and proof boundaries.
- **Ken natural order** — `Data.Numeric.Nat.Order` — the canonical strict
  comparison and its checked non-strict bridges.

## Trust & derivation

**Public API:** `argmin`, its seven §1.2 theorems,
`argmin_cost_order_invariant`, `argmin_append_keeps_left` and
`argmin_append_takes_right`; `argmin_option` and
its four §1.4 laws; `argmax` and the seven §3.2 laws `argmax_member`,
`argmax_greatest`, `argmax_no_costlier`, `argmax_head_on_tie`,
`argmax_singleton`, `argmax_keeps_head` and `argmax_takes_tail`, plus
`argmax_cost_order_invariant`; and `argmax_option` with its four §3.4 laws
`argmax_option_nil`,
`argmax_option_cons`, `argmax_option_member` and `argmax_option_greatest`.
Selection helpers, branch proofs, the shared membership induction and all
Option proof helpers are private. The carrier, list view, membership predicate and
Nat order retain their provider identities.

| Reader task | Section |
|---|---|
| Inspect the selector and public laws | [Definition](#definition) |
| Construct a valid input | [Using it](#using-it) |
| Review membership and optimality | [Laws & proofs](#laws--proofs) |
| Locate trust and dependencies | [Trust & derivation](#trust--derivation) |

**Derivation:** `NonEmpty` and its checked list-view equality are provided by
`Data.Collections.NonEmpty`; `list_elem` and its introduction lemmas by
`Data.Collections.Derived`; `nonempty_append` and its list-view proof by
`Data.Collections.NonEmpty`; `cong`, `sym` and `trans` by
`Core.Logic.Transport`; `IsTrue` and `leq_nat` by
`Core.Classes.LawfulClasses`; `comp` by `Core.Function.Combinators`; and
`lt_nat` with its two attached bridges by `Data.Numeric.Nat.Order`. The
selectors add no axiom, primitive, postulate or opaque constant.
`trusted_base()` must stay unchanged relative to the provider closure;
this does not erase inherited assumptions.

**Validation:** `conformance/stdlib/optimization/seed-argmin.md` names
independent singleton, strict-minimum and earliest-tie observations, law
instances and false-result refusals. Its scoped acceptance target checks the
client programs and the generic seven law types. The
[§1.5 cost-order seed](../../../../conformance/stdlib/optimization/seed-argmin-cost-order-invariance.md)
adds a four-label non-head earliest tie, an open strict-order proof,
and a flattened-cost false result on the same input. The candidate's
`cat_optimization_argmin_cost_order` package test root-loads the
checked public law, checks the whole accepted seed client and refuses
the isolated false claim. The
[§1.6 append-split seed](../../../../conformance/stdlib/optimization/seed-argmin-append-split.md)
exercises the tied-left and strictly-cheaper-right cases; its package-owned
test checks the complete accepted client and refuses the isolated later-tie
claim. These are candidate-local checks, not official conformance-runner
execution, CI, publication, landing or landed trust.
`conformance/stdlib/optimization/seed-argmin-option.md` adds empty,
singleton and non-degenerate tie cases, instances of the four Option laws,
and an isolated false `Some`-on-`Nil` refusal. The Option target root-loads
the public declarations, executes those cases and compares exact
trusted-base identities with the same provider closure.
`conformance/stdlib/optimization/seed-argmax.md` supplies an earliest-tied
maximum and a false later-tie result; its package test checks the full
positive client before refusing the false equality.
`conformance/stdlib/optimization/seed-argmax-option.md` supplies empty,
singleton and non-head earliest-tie cases, instances of all four Option laws,
and five isolated false-result refusals. Its package test checks the full
positive client and then each refusal against a fresh accepted prefix. The
[§3.5 cost-order seed](../../../../conformance/stdlib/optimization/seed-argmax-cost-order-invariance.md)
adds four labelled costs with a non-head earliest tied maximum, an open
strict-order proof, both independently checked shifted/original selections,
and a constant-cost false equality. The package-owned §3.5 test root-loads the
checked generic law and the complete accepted seed before refusing the
isolated false claim. These are candidate-local checks, not official
conformance-runner execution, CI, publication, landing or landed trust.
