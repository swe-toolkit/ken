# Persistent leftist priority queue

A priority queue stores priorities separately from payloads and extracts a
minimum under one explicit lawful order. This implementation is persistent:
insertions, merges, and removals return new queues without consuming their
inputs.

## Contents

1. [Motivation](#motivation)
2. [Definition](#definition)
3. [Using it](#using-it)
4. [Laws and proofs](#laws-and-proofs)
5. [Design notes](#design-notes)
6. [References](#references)
7. [Trust and derivation](#trust-and-derivation)

## Motivation

A priority and its payload play different roles. The order compares priorities
only, so equal-priority payloads remain distinct and duplicate entries retain
their multiplicity. The queue type records the comparator function projected
from its `Ord` dictionary, preventing queues with incompatible orders from being
merged.

The public carrier is abstract. Its constructors, cached ranks, balancing helper,
and structural meld worker remain private, leaving clients with five persistent
operations.

## Definition

A node caches one plus its right child's rank. `make_node` puts the child with
the greater rank on the left, maintaining the leftist shape locally. `meld`
chooses the lesser root, descends through one right child, and rebuilds through
that balancing helper.

```ken
import Core.Classes.LawfulClasses
  (Ord, IsTrue, bool_and, bool_or, bool_cases, ord_leq_at, leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Data.Collections.Derived (length)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (min as nat_min, sub as nat_sub)

pub data PriorityQueue (k : Type) (v : Type) (leq : k → k → Bool) : Type where {
  Empty : PriorityQueue k v leq;
  Node : Nat → k → v → PriorityQueue k v leq → PriorityQueue k v leq → PriorityQueue k v leq
}

fn rank (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq) : Nat =
  match q {
    Empty ↦ Zero;
    Node cached priority payload left right ↦ cached
  }

fn make_node
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
    : PriorityQueue k v leq =
  match leq_nat (rank k v leq left) (rank k v leq right) {
    True ↦ Node k v leq (Suc (rank k v leq left)) priority payload right left;
    False ↦ Node k v leq (Suc (rank k v leq right)) priority payload left right
  }

fn meld
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (first : PriorityQueue k v leq)
      (second : PriorityQueue k v leq)
    : PriorityQueue k v leq =
  match first {
    Empty ↦ second;
    Node first_rank first_priority first_payload first_left first_right ↦
      match second {
        Empty ↦ first;
        Node second_rank second_priority second_payload second_left second_right ↦
          match leq first_priority second_priority {
            True ↦
              make_node
                k
                v
                leq
                first_priority
                first_payload
                first_left
                (meld k v leq first_right second);
            False ↦
              make_node
                k
                v
                leq
                second_priority
                second_payload
                second_left
                (meld k v leq first second_right)
          }
      }
  }

pub fn empty (k : Type) (v : Type) (d : Ord k) : PriorityQueue k v (ord_leq_at k d) =
  Empty k v (ord_leq_at k d)

pub fn merge
      (k : Type)
      (v : Type)
      (d : Ord k)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
    : PriorityQueue k v (ord_leq_at k d) =
  meld k v (ord_leq_at k d) left right

pub fn insert
      (k : Type)
      (v : Type)
      (d : Ord k)
      (priority : k)
      (payload : v)
      (q : PriorityQueue k v (ord_leq_at k d))
    : PriorityQueue k v (ord_leq_at k d) =
  merge
    k
    v
    d
    (Node
      k
      v
      (ord_leq_at k d)
      (Suc Zero)
      priority
      payload
      (Empty k v (ord_leq_at k d))
      (Empty k v (ord_leq_at k d)))
    q

pub fn find_min
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Option (Pair k v) =
  match q {
    Empty ↦ None (Pair k v);
    Node cached priority payload left right ↦ Some (Pair k v) (mk_pair k v priority payload)
  }

pub fn pop_min
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))) =
  match q {
    Empty ↦ None (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)));
    Node cached priority payload left right ↦
      Some
        (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
        (mk_pair
          (Pair k v)
          (PriorityQueue k v (ord_leq_at k d))
          (mk_pair k v priority payload)
          (merge k v d left right))
  }

fn contribution (b : Bool) : Nat =
  match b {
    True ↦ Suc Zero;
    False ↦ Zero
  }

fn count_by
      (k : Type) (v : Type) (leq : k → k → Bool) (p : k → v → Bool) (q : PriorityQueue k v leq)
    : Nat =
  match q {
    Empty ↦ Zero;
    Node cached priority payload left right ↦
      add
        (contribution (p priority payload))
        (add (count_by k v leq p left) (count_by k v leq p right))
  }

fn size (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq) : Nat =
  count_by k v leq (λpriority. λpayload. True) q

fn root_bound
      (k : Type) (v : Type) (leq : k → k → Bool) (bound : k) (q : PriorityQueue k v leq)
    : Bool =
  match q {
    Empty ↦ True;
    Node cached priority payload left right ↦ leq bound priority
  }

fn all_above
      (k : Type) (v : Type) (leq : k → k → Bool) (bound : k) (q : PriorityQueue k v leq)
    : Bool =
  match q {
    Empty ↦ True;
    Node cached priority payload left right ↦
      bool_and
        (leq bound priority)
        (bool_and (all_above k v leq bound left) (all_above k v leq bound right))
  }

fn valid_bool (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq) : Bool =
  match q {
    Empty ↦ True;
    Node cached priority payload left right ↦
      bool_and
        (valid_bool k v leq left)
        (bool_and
          (valid_bool k v leq right)
          (bool_and
            (root_bound k v leq priority left)
            (bool_and
              (root_bound k v leq priority right)
              (bool_and
                (leq_nat (rank k v leq right) (rank k v leq left))
                (bool_and
                  (leq_nat cached (Suc (rank k v leq right)))
                  (leq_nat (Suc (rank k v leq right)) cached))))))
  }

fn Valid (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq) : Prop =
  IsTrue (valid_bool k v leq q)

fn right_spine_length
      (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq)
    : Nat =
  match q {
    Empty ↦ Zero;
    Node cached priority payload left right ↦ Suc (right_spine_length k v leq right)
  }

fn observe_pops
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : Pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) =
  match n {
    Zero ↦ mk_pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) (Nil (Pair k v)) q;
    Suc remaining ↦
      match pop_min k v d q {
        None ↦
          mk_pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) (Nil (Pair k v)) q;
        Some step ↦
          let
            entry = pair_fst (Pair k v) (PriorityQueue k v (ord_leq_at k d)) step;
            next = pair_snd (Pair k v) (PriorityQueue k v (ord_leq_at k d)) step;
            observed = observe_pops k v d remaining next
          in
            mk_pair
              (List (Pair k v))
              (PriorityQueue k v (ord_leq_at k d))
              (Cons
                (Pair k v)
                entry
                (pair_fst (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) observed))
              (pair_snd (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) observed)
      }
  }

theorem add_selected_first
      (root : Nat) (left : Nat) (right : Nat) (other : Nat)
    : Equal Nat
        (add root (add left (add right other)))
        (add (add root (add left right)) other) =
  trans
    Nat
    (add root (add left (add right other)))
    (add root (add (add left right) other))
    (add (add root (add left right)) other)
    (cong
      Nat
      Nat
      (add left (add right other))
      (add (add left right) other)
      (λn. add root n)
      ((proof assoc for add) left right other))
    ((proof assoc for add) root (add left right) other)

theorem add_selected_second
      (first : Nat) (root : Nat) (left : Nat) (right : Nat)
    : Equal Nat
        (add root (add left (add first right)))
        (add first (add root (add left right))) =
  trans
    Nat
    (add root (add left (add first right)))
    (add root (add (add left first) right))
    (add first (add root (add left right)))
    (cong
      Nat
      Nat
      (add left (add first right))
      (add (add left first) right)
      (λn. add root n)
      ((proof assoc for add) left first right))
    (trans
      Nat
      (add root (add (add left first) right))
      (add (add root (add left first)) right)
      (add first (add root (add left right)))
      ((proof assoc for add) root (add left first) right)
      (trans
        Nat
        (add (add root (add left first)) right)
        (add (add (add root left) first) right)
        (add first (add root (add left right)))
        (cong
          Nat
          Nat
          (add root (add left first))
          (add (add root left) first)
          (λn. add n right)
          ((proof assoc for add) root left first))
        (trans
          Nat
          (add (add (add root left) first) right)
          (add (add first (add root left)) right)
          (add first (add root (add left right)))
          (cong
            Nat
            Nat
            (add (add root left) first)
            (add first (add root left))
            (λn. add n right)
            ((proof comm for add) (add root left) first))
          (trans
            Nat
            (add (add first (add root left)) right)
            (add first (add (add root left) right))
            (add first (add root (add left right)))
            (sym
              Nat
              (add first (add (add root left) right))
              (add (add first (add root left)) right)
              ((proof assoc for add) first (add root left) right))
            (cong
              Nat
              Nat
              (add (add root left) right)
              (add root (add left right))
              (λn. add first n)
              (sym
                Nat
                (add root (add left right))
                (add (add root left) right)
                ((proof assoc for add) root left right)))))))

theorem make_node_count
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (p : k → v → Bool)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
    : Equal Nat
        (count_by k v leq p (make_node k v leq priority payload left right))
        (add
          (contribution (p priority payload))
          (add (count_by k v leq p left) (count_by k v leq p right))) =
  bool_cases
    (leq_nat (rank k v leq left) (rank k v leq right))
    (λb.
      Equal
        Nat
        (count_by
          k
          v
          leq
          p
          (match b {
            True ↦ Node k v leq (Suc (rank k v leq left)) priority payload right left;
            False ↦ Node k v leq (Suc (rank k v leq right)) priority payload left right
          }))
        (add
          (contribution (p priority payload))
          (add (count_by k v leq p left) (count_by k v leq p right))))
    (cong
      Nat
      Nat
      (add (count_by k v leq p right) (count_by k v leq p left))
      (add (count_by k v leq p left) (count_by k v leq p right))
      (λn. add (contribution (p priority payload)) n)
      ((proof comm for add) (count_by k v leq p right) (count_by k v leq p left)))
    Refl

theorem meld_count_selected_first
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (p : k → v → Bool)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v leq)
      (first_right : PriorityQueue k v leq)
      (second : PriorityQueue k v leq)
      (ih : Equal
        Nat
        (count_by k v leq p (meld k v leq first_right second))
        (add (count_by k v leq p first_right) (count_by k v leq p second)))
    : Equal Nat
        (count_by
          k
          v
          leq
          p
          (make_node
            k
            v
            leq
            first_priority
            first_payload
            first_left
            (meld k v leq first_right second)))
        (add
          (count_by
            k
            v
            leq
            p
            (Node k v leq first_rank first_priority first_payload first_left first_right))
          (count_by k v leq p second)) =
  trans
    Nat
    (count_by
      k
      v
      leq
      p
      (make_node
        k
        v
        leq
        first_priority
        first_payload
        first_left
        (meld k v leq first_right second)))
    (add
      (contribution (p first_priority first_payload))
      (add
        (count_by k v leq p first_left)
        (count_by k v leq p (meld k v leq first_right second))))
    (add
      (count_by
        k
        v
        leq
        p
        (Node k v leq first_rank first_priority first_payload first_left first_right))
      (count_by k v leq p second))
    (make_node_count
      k
      v
      leq
      p
      first_priority
      first_payload
      first_left
      (meld k v leq first_right second))
    (trans
      Nat
      (add
        (contribution (p first_priority first_payload))
        (add
          (count_by k v leq p first_left)
          (count_by k v leq p (meld k v leq first_right second))))
      (add
        (contribution (p first_priority first_payload))
        (add
          (count_by k v leq p first_left)
          (add (count_by k v leq p first_right) (count_by k v leq p second))))
      (add
        (count_by
          k
          v
          leq
          p
          (Node k v leq first_rank first_priority first_payload first_left first_right))
        (count_by k v leq p second))
      (cong
        Nat
        Nat
        (count_by k v leq p (meld k v leq first_right second))
        (add (count_by k v leq p first_right) (count_by k v leq p second))
        (λn.
          add
            (contribution (p first_priority first_payload))
            (add (count_by k v leq p first_left) n))
        ih)
      (add_selected_first
        (contribution (p first_priority first_payload))
        (count_by k v leq p first_left)
        (count_by k v leq p first_right)
        (count_by k v leq p second)))

theorem meld_count_selected_second
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (p : k → v → Bool)
      (first : PriorityQueue k v leq)
      (second_rank : Nat)
      (second_priority : k)
      (second_payload : v)
      (second_left : PriorityQueue k v leq)
      (second_right : PriorityQueue k v leq)
      (ih : Equal
        Nat
        (count_by k v leq p (meld k v leq first second_right))
        (add (count_by k v leq p first) (count_by k v leq p second_right)))
    : Equal Nat
        (count_by
          k
          v
          leq
          p
          (make_node
            k
            v
            leq
            second_priority
            second_payload
            second_left
            (meld k v leq first second_right)))
        (add
          (count_by k v leq p first)
          (count_by
            k
            v
            leq
            p
            (Node
              k
              v
              leq
              second_rank
              second_priority
              second_payload
              second_left
              second_right))) =
  trans
    Nat
    (count_by
      k
      v
      leq
      p
      (make_node
        k
        v
        leq
        second_priority
        second_payload
        second_left
        (meld k v leq first second_right)))
    (add
      (contribution (p second_priority second_payload))
      (add
        (count_by k v leq p second_left)
        (count_by k v leq p (meld k v leq first second_right))))
    (add
      (count_by k v leq p first)
      (count_by
        k
        v
        leq
        p
        (Node k v leq second_rank second_priority second_payload second_left second_right)))
    (make_node_count
      k
      v
      leq
      p
      second_priority
      second_payload
      second_left
      (meld k v leq first second_right))
    (trans
      Nat
      (add
        (contribution (p second_priority second_payload))
        (add
          (count_by k v leq p second_left)
          (count_by k v leq p (meld k v leq first second_right))))
      (add
        (contribution (p second_priority second_payload))
        (add
          (count_by k v leq p second_left)
          (add (count_by k v leq p first) (count_by k v leq p second_right))))
      (add
        (count_by k v leq p first)
        (count_by
          k
          v
          leq
          p
          (Node k v leq second_rank second_priority second_payload second_left second_right)))
      (cong
        Nat
        Nat
        (count_by k v leq p (meld k v leq first second_right))
        (add (count_by k v leq p first) (count_by k v leq p second_right))
        (λn.
          add
            (contribution (p second_priority second_payload))
            (add (count_by k v leq p second_left) n))
        ih)
      (add_selected_second
        (count_by k v leq p first)
        (contribution (p second_priority second_payload))
        (count_by k v leq p second_left)
        (count_by k v leq p second_right)))

theorem meld_count_node
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (p : k → v → Bool)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v leq)
      (first_right : PriorityQueue k v leq)
      (first_right_ih : (second : PriorityQueue k v leq)
        → Equal
        Nat
        (count_by k v leq p (meld k v leq first_right second))
        (add (count_by k v leq p first_right) (count_by k v leq p second)))
      (second : PriorityQueue k v leq)
    : Equal Nat
        (count_by
          k
          v
          leq
          p
          (meld
            k
            v
            leq
            (Node k v leq first_rank first_priority first_payload first_left first_right)
            second))
        (add
          (count_by
            k
            v
            leq
            p
            (Node k v leq first_rank first_priority first_payload first_left first_right))
          (count_by k v leq p second)) =
  match second {
    Empty ↦ Refl;
    Node second_rank second_priority second_payload second_left second_right ↦
      bool_cases
        (leq first_priority second_priority)
        (λchoice.
          Equal
            Nat
            (count_by
              k
              v
              leq
              p
              (match choice {
                True ↦
                  make_node
                    k
                    v
                    leq
                    first_priority
                    first_payload
                    first_left
                    (meld
                      k
                      v
                      leq
                      first_right
                      (Node
                        k
                        v
                        leq
                        second_rank
                        second_priority
                        second_payload
                        second_left
                        second_right));
                False ↦
                  make_node
                    k
                    v
                    leq
                    second_priority
                    second_payload
                    second_left
                    (meld
                      k
                      v
                      leq
                      (Node
                        k
                        v
                        leq
                        first_rank
                        first_priority
                        first_payload
                        first_left
                        first_right)
                      second_right)
              }))
            (add
              (count_by
                k
                v
                leq
                p
                (Node k v leq first_rank first_priority first_payload first_left first_right))
              (count_by
                k
                v
                leq
                p
                (Node
                  k
                  v
                  leq
                  second_rank
                  second_priority
                  second_payload
                  second_left
                  second_right))))
        (meld_count_selected_first
          k
          v
          leq
          p
          first_rank
          first_priority
          first_payload
          first_left
          first_right
          (Node k v leq second_rank second_priority second_payload second_left second_right)
          (first_right_ih
            (Node k v leq second_rank second_priority second_payload second_left second_right)))
        (meld_count_selected_second
          k
          v
          leq
          p
          (Node k v leq first_rank first_priority first_payload first_left first_right)
          second_rank
          second_priority
          second_payload
          second_left
          second_right
          (meld_count_node
            k
            v
            leq
            p
            first_rank
            first_priority
            first_payload
            first_left
            first_right
            first_right_ih
            second_right))
  }

theorem meld_count
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (p : k → v → Bool)
      (first : PriorityQueue k v leq)
    : (second : PriorityQueue k v leq)
      → Equal Nat
        (count_by k v leq p (meld k v leq first second))
        (add (count_by k v leq p first) (count_by k v leq p second)) =
  match first {
    Empty ↦
      λsecond.
        sym
          Nat
          (add Zero (count_by k v leq p second))
          (count_by k v leq p second)
          ((proof zero_l for add) (count_by k v leq p second));
    Node first_rank first_priority first_payload first_left first_right ↦
      meld_count_node
        k
        v
        leq
        p
        first_rank
        first_priority
        first_payload
        first_left
        first_right
        (meld_count k v leq p first_right)
  }

fn option_is_some (a : Type) (value : Option a) : Bool =
  match value {
    None ↦ False;
    Some item ↦ True
  }

theorem none_not_some
      (a : Type) (item : a) (same : Equal (Option a) (None a) (Some a item))
    : Bottom =
  same

theorem some_injective
      (a : Type) (left : a) (right : a) (same : Equal (Option a) (Some a left) (Some a right))
    : Equal a left right =
  same

theorem empty_count
      (k : Type) (v : Type) (d : Ord k) (p : k → v → Bool)
    : Equal Nat (count_by k v (ord_leq_at k d) p (empty k v d)) Zero =
  Proved

theorem merge_count
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (count_by k v (ord_leq_at k d) p (merge k v d left right))
        (add (count_by k v (ord_leq_at k d) p left) (count_by k v (ord_leq_at k d) p right)) =
  meld_count k v (ord_leq_at k d) p left right

theorem insert_count
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (priority : k)
      (payload : v)
      (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (count_by k v (ord_leq_at k d) p (insert k v d priority payload q))
        (add (contribution (p priority payload)) (count_by k v (ord_leq_at k d) p q)) =
  merge_count
    k
    v
    d
    p
    (Node
      k
      v
      (ord_leq_at k d)
      (Suc Zero)
      priority
      payload
      (Empty k v (ord_leq_at k d))
      (Empty k v (ord_leq_at k d)))
    q

theorem pop_count_transport
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
      (actual_remainder : PriorityQueue k v (ord_leq_at k d))
      (entry : Pair k v)
      (remainder : PriorityQueue k v (ord_leq_at k d))
      (priority_same : Equal k priority (pair_fst k v entry))
      (payload_same : Equal v payload (pair_snd k v entry))
      (remainder_same : Equal (PriorityQueue k v (ord_leq_at k d)) actual_remainder remainder)
      (base : Equal
        Nat
        (count_by
          k
          v
          (ord_leq_at k d)
          p
          (Node k v (ord_leq_at k d) cached priority payload left right))
        (add
          (contribution (p priority payload))
          (count_by k v (ord_leq_at k d) p actual_remainder)))
    : Equal Nat
        (count_by
          k
          v
          (ord_leq_at k d)
          p
          (Node k v (ord_leq_at k d) cached priority payload left right))
        (add
          (contribution (p (pair_fst k v entry) (pair_snd k v entry)))
          (count_by k v (ord_leq_at k d) p remainder)) =
  J
    (λout_priority _.
      Equal
        Nat
        (count_by
          k
          v
          (ord_leq_at k d)
          p
          (Node k v (ord_leq_at k d) cached priority payload left right))
        (add
          (contribution (p out_priority (pair_snd k v entry)))
          (count_by k v (ord_leq_at k d) p remainder)))
    (J
      (λout_payload _.
        Equal
          Nat
          (count_by
            k
            v
            (ord_leq_at k d)
            p
            (Node k v (ord_leq_at k d) cached priority payload left right))
          (add
            (contribution (p priority out_payload))
            (count_by k v (ord_leq_at k d) p remainder)))
      (J
        (λout_remainder _.
          Equal
            Nat
            (count_by
              k
              v
              (ord_leq_at k d)
              p
              (Node k v (ord_leq_at k d) cached priority payload left right))
            (add
              (contribution (p priority payload))
              (count_by k v (ord_leq_at k d) p out_remainder)))
        base
        remainder_same)
      payload_same)
    priority_same

theorem pop_min_count
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → Equal Nat
        (count_by k v (ord_leq_at k d) p q)
        (add
          (contribution (p (pair_fst k v entry) (pair_snd k v entry)))
          (count_by k v (ord_leq_at k d) p remainder)) =
  match q {
    Empty ↦
      λentry.
        λremainder.
          λsame.
            absurd
              (none_not_some
                (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
                (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder)
                same);
    Node cached priority payload left right ↦
      λentry.
        λremainder.
          λsame.
            pop_count_transport
              k
              v
              d
              p
              cached
              priority
              payload
              left
              right
              (merge k v d left right)
              entry
              remainder
              (and_fst
                (Equal k priority (pair_fst k v entry))
                (Equal v payload (pair_snd k v entry))
                (and_fst
                  (Equal (Pair k v) (mk_pair k v priority payload) entry)
                  (Equal
                    (PriorityQueue k v (ord_leq_at k d))
                    (merge k v d left right)
                    remainder)
                  same))
              (and_snd
                (Equal k priority (pair_fst k v entry))
                (Equal v payload (pair_snd k v entry))
                (and_fst
                  (Equal (Pair k v) (mk_pair k v priority payload) entry)
                  (Equal
                    (PriorityQueue k v (ord_leq_at k d))
                    (merge k v d left right)
                    remainder)
                  same))
              (and_snd
                (Equal (Pair k v) (mk_pair k v priority payload) entry)
                (Equal (PriorityQueue k v (ord_leq_at k d)) (merge k v d left right) remainder)
                same)
              (cong
                Nat
                Nat
                (add
                  (count_by k v (ord_leq_at k d) p left)
                  (count_by k v (ord_leq_at k d) p right))
                (count_by k v (ord_leq_at k d) p (merge k v d left right))
                (λn. add (contribution (p priority payload)) n)
                (sym
                  Nat
                  (count_by k v (ord_leq_at k d) p (merge k v d left right))
                  (add
                    (count_by k v (ord_leq_at k d) p left)
                    (count_by k v (ord_leq_at k d) p right))
                  (merge_count k v d p left right)))
  }

theorem valid_node_intro
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (left_valid : Valid k v leq left)
      (right_valid : Valid k v leq right)
      (left_bound : IsTrue (root_bound k v leq priority left))
      (right_bound : IsTrue (root_bound k v leq priority right))
      (balanced : IsTrue (leq_nat (rank k v leq right) (rank k v leq left)))
      (cache_forward : IsTrue (leq_nat cached (Suc (rank k v leq right))))
      (cache_reverse : IsTrue (leq_nat (Suc (rank k v leq right)) cached))
    : Valid k v leq (Node k v leq cached priority payload left right) =
  (proof intro for bool_and)
    (valid_bool k v leq left)
    (bool_and
      (valid_bool k v leq right)
      (bool_and
        (root_bound k v leq priority left)
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached))))))
    left_valid
    ((proof intro for bool_and)
      (valid_bool k v leq right)
      (bool_and
        (root_bound k v leq priority left)
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached)))))
      right_valid
      ((proof intro for bool_and)
        (root_bound k v leq priority left)
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached))))
        left_bound
        ((proof intro for bool_and)
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached)))
          right_bound
          ((proof intro for bool_and)
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached))
            balanced
            ((proof intro for bool_and)
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached)
              cache_forward
              cache_reverse)))))

theorem valid_node_left
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : Valid k v leq left =
  (proof left for bool_and)
    (valid_bool k v leq left)
    (bool_and
      (valid_bool k v leq right)
      (bool_and
        (root_bound k v leq priority left)
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached))))))
    valid

theorem valid_node_tail
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue
        (bool_and
          (valid_bool k v leq right)
          (bool_and
            (root_bound k v leq priority left)
            (bool_and
              (root_bound k v leq priority right)
              (bool_and
                (leq_nat (rank k v leq right) (rank k v leq left))
                (bool_and
                  (leq_nat cached (Suc (rank k v leq right)))
                  (leq_nat (Suc (rank k v leq right)) cached)))))) =
  (proof right for bool_and)
    (valid_bool k v leq left)
    (bool_and
      (valid_bool k v leq right)
      (bool_and
        (root_bound k v leq priority left)
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached))))))
    valid

theorem valid_node_right
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : Valid k v leq right =
  (proof left for bool_and)
    (valid_bool k v leq right)
    (bool_and
      (root_bound k v leq priority left)
      (bool_and
        (root_bound k v leq priority right)
        (bool_and
          (leq_nat (rank k v leq right) (rank k v leq left))
          (bool_and
            (leq_nat cached (Suc (rank k v leq right)))
            (leq_nat (Suc (rank k v leq right)) cached)))))
    (valid_node_tail k v leq cached priority payload left right valid)

theorem valid_node_after_right
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue
        (bool_and
          (root_bound k v leq priority left)
          (bool_and
            (root_bound k v leq priority right)
            (bool_and
              (leq_nat (rank k v leq right) (rank k v leq left))
              (bool_and
                (leq_nat cached (Suc (rank k v leq right)))
                (leq_nat (Suc (rank k v leq right)) cached))))) =
  (proof right for bool_and)
    (valid_bool k v leq right)
    (bool_and
      (root_bound k v leq priority left)
      (bool_and
        (root_bound k v leq priority right)
        (bool_and
          (leq_nat (rank k v leq right) (rank k v leq left))
          (bool_and
            (leq_nat cached (Suc (rank k v leq right)))
            (leq_nat (Suc (rank k v leq right)) cached)))))
    (valid_node_tail k v leq cached priority payload left right valid)

theorem valid_node_left_bound
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue (root_bound k v leq priority left) =
  (proof left for bool_and)
    (root_bound k v leq priority left)
    (bool_and
      (root_bound k v leq priority right)
      (bool_and
        (leq_nat (rank k v leq right) (rank k v leq left))
        (bool_and
          (leq_nat cached (Suc (rank k v leq right)))
          (leq_nat (Suc (rank k v leq right)) cached))))
    (valid_node_after_right k v leq cached priority payload left right valid)

theorem valid_node_after_left_bound
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue
        (bool_and
          (root_bound k v leq priority right)
          (bool_and
            (leq_nat (rank k v leq right) (rank k v leq left))
            (bool_and
              (leq_nat cached (Suc (rank k v leq right)))
              (leq_nat (Suc (rank k v leq right)) cached)))) =
  (proof right for bool_and)
    (root_bound k v leq priority left)
    (bool_and
      (root_bound k v leq priority right)
      (bool_and
        (leq_nat (rank k v leq right) (rank k v leq left))
        (bool_and
          (leq_nat cached (Suc (rank k v leq right)))
          (leq_nat (Suc (rank k v leq right)) cached))))
    (valid_node_after_right k v leq cached priority payload left right valid)

theorem valid_node_right_bound
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue (root_bound k v leq priority right) =
  (proof left for bool_and)
    (root_bound k v leq priority right)
    (bool_and
      (leq_nat (rank k v leq right) (rank k v leq left))
      (bool_and
        (leq_nat cached (Suc (rank k v leq right)))
        (leq_nat (Suc (rank k v leq right)) cached)))
    (valid_node_after_left_bound k v leq cached priority payload left right valid)

theorem valid_node_rank_tail
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue
        (bool_and
          (leq_nat (rank k v leq right) (rank k v leq left))
          (bool_and
            (leq_nat cached (Suc (rank k v leq right)))
            (leq_nat (Suc (rank k v leq right)) cached))) =
  (proof right for bool_and)
    (root_bound k v leq priority right)
    (bool_and
      (leq_nat (rank k v leq right) (rank k v leq left))
      (bool_and
        (leq_nat cached (Suc (rank k v leq right)))
        (leq_nat (Suc (rank k v leq right)) cached)))
    (valid_node_after_left_bound k v leq cached priority payload left right valid)

theorem valid_node_balance
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue (leq_nat (rank k v leq right) (rank k v leq left)) =
  (proof left for bool_and)
    (leq_nat (rank k v leq right) (rank k v leq left))
    (bool_and
      (leq_nat cached (Suc (rank k v leq right)))
      (leq_nat (Suc (rank k v leq right)) cached))
    (valid_node_rank_tail k v leq cached priority payload left right valid)

theorem valid_node_cache_pair
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : IsTrue
        (bool_and
          (leq_nat cached (Suc (rank k v leq right)))
          (leq_nat (Suc (rank k v leq right)) cached)) =
  (proof right for bool_and)
    (leq_nat (rank k v leq right) (rank k v leq left))
    (bool_and
      (leq_nat cached (Suc (rank k v leq right)))
      (leq_nat (Suc (rank k v leq right)) cached))
    (valid_node_rank_tail k v leq cached priority payload left right valid)

theorem valid_node_cache_equal
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (valid : Valid k v leq (Node k v leq cached priority payload left right))
    : Equal Nat cached (Suc (rank k v leq right)) =
  (proof antisym for leq_nat)
    cached
    (Suc (rank k v leq right))
    ((proof left for bool_and)
      (leq_nat cached (Suc (rank k v leq right)))
      (leq_nat (Suc (rank k v leq right)) cached)
      (valid_node_cache_pair k v leq cached priority payload left right valid))
    ((proof right for bool_and)
      (leq_nat cached (Suc (rank k v leq right)))
      (leq_nat (Suc (rank k v leq right)) cached)
      (valid_node_cache_pair k v leq cached priority payload left right valid))

theorem reverse_leq_nat_of_false
      (left : Nat) (right : Nat) (forward_false : Equal Bool (leq_nat left right) False)
    : IsTrue (leq_nat right left) =
  (proof left_false_elim for bool_or)
    (leq_nat left right)
    (leq_nat right left)
    forward_false
    ((proof total for leq_nat) left right)

theorem make_node_valid
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (left_valid : Valid k v leq left)
      (right_valid : Valid k v leq right)
      (left_bound : IsTrue (root_bound k v leq priority left))
      (right_bound : IsTrue (root_bound k v leq priority right))
    : Valid k v leq (make_node k v leq priority payload left right) =
  bool_cases
    (leq_nat (rank k v leq left) (rank k v leq right))
    (λchoice.
      Equal Bool (leq_nat (rank k v leq left) (rank k v leq right)) choice
      → Valid
        k
        v
        leq
        (match choice {
          True ↦ Node k v leq (Suc (rank k v leq left)) priority payload right left;
          False ↦ Node k v leq (Suc (rank k v leq right)) priority payload left right
        }))
    (λcompared.
      valid_node_intro
        k
        v
        leq
        (Suc (rank k v leq left))
        priority
        payload
        right
        left
        right_valid
        left_valid
        right_bound
        left_bound
        compared
        ((proof refl for leq_nat) (Suc (rank k v leq left)))
        ((proof refl for leq_nat) (Suc (rank k v leq left))))
    (λcompared.
      valid_node_intro
        k
        v
        leq
        (Suc (rank k v leq right))
        priority
        payload
        left
        right
        left_valid
        right_valid
        left_bound
        right_bound
        (reverse_leq_nat_of_false (rank k v leq left) (rank k v leq right) compared)
        ((proof refl for leq_nat) (Suc (rank k v leq right)))
        ((proof refl for leq_nat) (Suc (rank k v leq right))))
    Refl

theorem empty_valid
      (k : Type) (v : Type) (d : Ord k)
    : Valid k v (ord_leq_at k d) (empty k v d) =
  Proved

theorem singleton_valid
      (k : Type) (v : Type) (d : Ord k) (priority : k) (payload : v)
    : Valid k v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          (Suc Zero)
          priority
          payload
          (Empty k v (ord_leq_at k d))
          (Empty k v (ord_leq_at k d))) =
  Proved

theorem valid_rank_shape
      (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq)
    : Valid k v leq q → Equal Nat (rank k v leq q) (right_spine_length k v leq q) =
  match q {
    Empty ↦ λvalid. Proved;
    Node cached priority payload left right ↦
      λvalid.
        trans
          Nat
          cached
          (Suc (rank k v leq right))
          (Suc (right_spine_length k v leq right))
          (valid_node_cache_equal k v leq cached priority payload left right valid)
          (cong
            Nat
            Nat
            (rank k v leq right)
            (right_spine_length k v leq right)
            Suc
            (valid_rank_shape
              k
              v
              leq
              right
              (valid_node_right k v leq cached priority payload left right valid)))
  }

theorem all_above_node_intro
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (root : IsTrue (leq bound priority))
      (left_above : IsTrue (all_above k v leq bound left))
      (right_above : IsTrue (all_above k v leq bound right))
    : IsTrue (all_above k v leq bound (Node k v leq cached priority payload left right)) =
  (proof intro for bool_and)
    (leq bound priority)
    (bool_and (all_above k v leq bound left) (all_above k v leq bound right))
    root
    ((proof intro for bool_and)
      (all_above k v leq bound left)
      (all_above k v leq bound right)
      left_above
      right_above)

theorem all_above_node_root
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (above : IsTrue
        (all_above k v leq bound (Node k v leq cached priority payload left right)))
    : IsTrue (leq bound priority) =
  (proof left for bool_and)
    (leq bound priority)
    (bool_and (all_above k v leq bound left) (all_above k v leq bound right))
    above

theorem all_above_node_children
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (above : IsTrue
        (all_above k v leq bound (Node k v leq cached priority payload left right)))
    : IsTrue (bool_and (all_above k v leq bound left) (all_above k v leq bound right)) =
  (proof right for bool_and)
    (leq bound priority)
    (bool_and (all_above k v leq bound left) (all_above k v leq bound right))
    above

theorem all_above_node_left
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (above : IsTrue
        (all_above k v leq bound (Node k v leq cached priority payload left right)))
    : IsTrue (all_above k v leq bound left) =
  (proof left for bool_and)
    (all_above k v leq bound left)
    (all_above k v leq bound right)
    (all_above_node_children k v leq bound cached priority payload left right above)

theorem all_above_node_right
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (above : IsTrue
        (all_above k v leq bound (Node k v leq cached priority payload left right)))
    : IsTrue (all_above k v leq bound right) =
  (proof right for bool_and)
    (all_above k v leq bound left)
    (all_above k v leq bound right)
    (all_above_node_children k v leq bound cached priority payload left right above)

theorem all_above_root_bound
      (k : Type) (v : Type) (leq : k → k → Bool) (bound : k) (q : PriorityQueue k v leq)
    : IsTrue (all_above k v leq bound q) → IsTrue (root_bound k v leq bound q) =
  match q {
    Empty ↦ λabove. Proved;
    Node cached priority payload left right ↦
      λabove. all_above_node_root k v leq bound cached priority payload left right above
  }

theorem root_bound_trans
      (k : Type)
      (v : Type)
      (d : Ord k)
      (lower : k)
      (upper : k)
      (q : PriorityQueue k v (ord_leq_at k d))
      (lower_upper : IsTrue (ord_leq_at k d lower upper))
    : IsTrue (root_bound k v (ord_leq_at k d) upper q)
      → IsTrue (root_bound k v (ord_leq_at k d) lower q) =
  match q {
    Empty ↦ λupper_root. Proved;
    Node cached priority payload left right ↦
      λupper_root. d.trans lower upper priority lower_upper upper_root
  }

theorem valid_root_global
      (k : Type) (v : Type) (d : Ord k) (bound : k) (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q
      → IsTrue (root_bound k v (ord_leq_at k d) bound q)
      → IsTrue (all_above k v (ord_leq_at k d) bound q) =
  match q {
    Empty ↦ λvalid. λbound_root. Proved;
    Node cached priority payload left right ↦
      λvalid.
        λbound_root.
          all_above_node_intro
            k
            v
            (ord_leq_at k d)
            bound
            cached
            priority
            payload
            left
            right
            bound_root
            (valid_root_global
              k
              v
              d
              bound
              left
              (valid_node_left k v (ord_leq_at k d) cached priority payload left right valid)
              (root_bound_trans
                k
                v
                d
                bound
                priority
                left
                bound_root
                (valid_node_left_bound
                  k
                  v
                  (ord_leq_at k d)
                  cached
                  priority
                  payload
                  left
                  right
                  valid)))
            (valid_root_global
              k
              v
              d
              bound
              right
              (valid_node_right k v (ord_leq_at k d) cached priority payload left right valid)
              (root_bound_trans
                k
                v
                d
                bound
                priority
                right
                bound_root
                (valid_node_right_bound
                  k
                  v
                  (ord_leq_at k d)
                  cached
                  priority
                  payload
                  left
                  right
                  valid)))
  }

theorem valid_all_above_root
      (k : Type)
      (v : Type)
      (d : Ord k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
      (valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node k v (ord_leq_at k d) cached priority payload left right))
    : IsTrue
        (all_above
          k
          v
          (ord_leq_at k d)
          priority
          (Node k v (ord_leq_at k d) cached priority payload left right)) =
  valid_root_global
    k
    v
    d
    priority
    (Node k v (ord_leq_at k d) cached priority payload left right)
    valid
    (d.refl priority)

theorem make_node_all_above
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v leq)
      (right : PriorityQueue k v leq)
      (root : IsTrue (leq bound priority))
      (left_above : IsTrue (all_above k v leq bound left))
      (right_above : IsTrue (all_above k v leq bound right))
    : IsTrue (all_above k v leq bound (make_node k v leq priority payload left right)) =
  bool_cases
    (leq_nat (rank k v leq left) (rank k v leq right))
    (λchoice.
      IsTrue
        (all_above
          k
          v
          leq
          bound
          (match choice {
            True ↦ Node k v leq (Suc (rank k v leq left)) priority payload right left;
            False ↦ Node k v leq (Suc (rank k v leq right)) priority payload left right
          })))
    (all_above_node_intro
      k
      v
      leq
      bound
      (Suc (rank k v leq left))
      priority
      payload
      right
      left
      root
      right_above
      left_above)
    (all_above_node_intro
      k
      v
      leq
      bound
      (Suc (rank k v leq right))
      priority
      payload
      left
      right
      root
      left_above
      right_above)

theorem meld_all_above_node
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (bound : k)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v leq)
      (first_right : PriorityQueue k v leq)
      (first_above : IsTrue
        (all_above
          k
          v
          leq
          bound
          (Node k v leq first_rank first_priority first_payload first_left first_right)))
      (first_right_ih : (second : PriorityQueue k v leq)
        → IsTrue
        (all_above k v leq bound second)
        → IsTrue
        (all_above k v leq bound (meld k v leq first_right second)))
      (second : PriorityQueue k v leq)
    : IsTrue (all_above k v leq bound second)
      → IsTrue
        (all_above
          k
          v
          leq
          bound
          (meld
            k
            v
            leq
            (Node k v leq first_rank first_priority first_payload first_left first_right)
            second)) =
  match second {
    Empty ↦ λsecond_above. first_above;
    Node second_rank second_priority second_payload second_left second_right ↦
      λsecond_above.
        bool_cases
          (leq first_priority second_priority)
          (λchoice.
            IsTrue
              (all_above
                k
                v
                leq
                bound
                (match choice {
                  True ↦
                    make_node
                      k
                      v
                      leq
                      first_priority
                      first_payload
                      first_left
                      (meld
                        k
                        v
                        leq
                        first_right
                        (Node
                          k
                          v
                          leq
                          second_rank
                          second_priority
                          second_payload
                          second_left
                          second_right));
                  False ↦
                    make_node
                      k
                      v
                      leq
                      second_priority
                      second_payload
                      second_left
                      (meld
                        k
                        v
                        leq
                        (Node
                          k
                          v
                          leq
                          first_rank
                          first_priority
                          first_payload
                          first_left
                          first_right)
                        second_right)
                })))
          (make_node_all_above
            k
            v
            leq
            bound
            first_priority
            first_payload
            first_left
            (meld
              k
              v
              leq
              first_right
              (Node
                k
                v
                leq
                second_rank
                second_priority
                second_payload
                second_left
                second_right))
            (all_above_node_root
              k
              v
              leq
              bound
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              first_above)
            (all_above_node_left
              k
              v
              leq
              bound
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              first_above)
            (first_right_ih
              (Node k v leq second_rank second_priority second_payload second_left second_right)
              second_above))
          (make_node_all_above
            k
            v
            leq
            bound
            second_priority
            second_payload
            second_left
            (meld
              k
              v
              leq
              (Node k v leq first_rank first_priority first_payload first_left first_right)
              second_right)
            (all_above_node_root
              k
              v
              leq
              bound
              second_rank
              second_priority
              second_payload
              second_left
              second_right
              second_above)
            (all_above_node_left
              k
              v
              leq
              bound
              second_rank
              second_priority
              second_payload
              second_left
              second_right
              second_above)
            (meld_all_above_node
              k
              v
              leq
              bound
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              first_above
              first_right_ih
              second_right
              (all_above_node_right
                k
                v
                leq
                bound
                second_rank
                second_priority
                second_payload
                second_left
                second_right
                second_above)))
  }

theorem meld_all_above
      (k : Type) (v : Type) (leq : k → k → Bool) (bound : k) (first : PriorityQueue k v leq)
    : IsTrue (all_above k v leq bound first)
      → (second : PriorityQueue k v leq)
      → IsTrue (all_above k v leq bound second)
      → IsTrue (all_above k v leq bound (meld k v leq first second)) =
  match first {
    Empty ↦ λfirst_above. λsecond. λsecond_above. second_above;
    Node first_rank first_priority first_payload first_left first_right ↦
      λfirst_above.
        meld_all_above_node
          k
          v
          leq
          bound
          first_rank
          first_priority
          first_payload
          first_left
          first_right
          first_above
          (meld_all_above
            k
            v
            leq
            bound
            first_right
            (all_above_node_right
              k
              v
              leq
              bound
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              first_above))
  }

theorem meld_valid_selected_first
      (k : Type)
      (v : Type)
      (d : Ord k)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v (ord_leq_at k d))
      (first_right : PriorityQueue k v (ord_leq_at k d))
      (second_rank : Nat)
      (second_priority : k)
      (second_payload : v)
      (second_left : PriorityQueue k v (ord_leq_at k d))
      (second_right : PriorityQueue k v (ord_leq_at k d))
      (compared : IsTrue (ord_leq_at k d first_priority second_priority))
      (first_valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          first_rank
          first_priority
          first_payload
          first_left
          first_right))
      (second_valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          second_rank
          second_priority
          second_payload
          second_left
          second_right))
      (recursive_valid : Valid
        k
        v
        (ord_leq_at k d)
        (meld
          k
          v
          (ord_leq_at k d)
          first_right
          (Node
            k
            v
            (ord_leq_at k d)
            second_rank
            second_priority
            second_payload
            second_left
            second_right)))
    : Valid k v
        (ord_leq_at k d)
        (make_node
          k
          v
          (ord_leq_at k d)
          first_priority
          first_payload
          first_left
          (meld
            k
            v
            (ord_leq_at k d)
            first_right
            (Node
              k
              v
              (ord_leq_at k d)
              second_rank
              second_priority
              second_payload
              second_left
              second_right))) =
  make_node_valid
    k
    v
    (ord_leq_at k d)
    first_priority
    first_payload
    first_left
    (meld
      k
      v
      (ord_leq_at k d)
      first_right
      (Node
        k
        v
        (ord_leq_at k d)
        second_rank
        second_priority
        second_payload
        second_left
        second_right))
    (valid_node_left
      k
      v
      (ord_leq_at k d)
      first_rank
      first_priority
      first_payload
      first_left
      first_right
      first_valid)
    recursive_valid
    (valid_node_left_bound
      k
      v
      (ord_leq_at k d)
      first_rank
      first_priority
      first_payload
      first_left
      first_right
      first_valid)
    (all_above_root_bound
      k
      v
      (ord_leq_at k d)
      first_priority
      (meld
        k
        v
        (ord_leq_at k d)
        first_right
        (Node
          k
          v
          (ord_leq_at k d)
          second_rank
          second_priority
          second_payload
          second_left
          second_right))
      (meld_all_above
        k
        v
        (ord_leq_at k d)
        first_priority
        first_right
        (all_above_node_right
          k
          v
          (ord_leq_at k d)
          first_priority
          first_rank
          first_priority
          first_payload
          first_left
          first_right
          (valid_all_above_root
            k
            v
            d
            first_rank
            first_priority
            first_payload
            first_left
            first_right
            first_valid))
        (Node
          k
          v
          (ord_leq_at k d)
          second_rank
          second_priority
          second_payload
          second_left
          second_right)
        (valid_root_global
          k
          v
          d
          first_priority
          (Node
            k
            v
            (ord_leq_at k d)
            second_rank
            second_priority
            second_payload
            second_left
            second_right)
          second_valid
          compared)))

theorem meld_valid_selected_second
      (k : Type)
      (v : Type)
      (d : Ord k)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v (ord_leq_at k d))
      (first_right : PriorityQueue k v (ord_leq_at k d))
      (second_rank : Nat)
      (second_priority : k)
      (second_payload : v)
      (second_left : PriorityQueue k v (ord_leq_at k d))
      (second_right : PriorityQueue k v (ord_leq_at k d))
      (compared : Equal Bool (ord_leq_at k d first_priority second_priority) False)
      (first_valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          first_rank
          first_priority
          first_payload
          first_left
          first_right))
      (second_valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          second_rank
          second_priority
          second_payload
          second_left
          second_right))
      (recursive_valid : Valid
        k
        v
        (ord_leq_at k d)
        (meld
          k
          v
          (ord_leq_at k d)
          (Node
            k
            v
            (ord_leq_at k d)
            first_rank
            first_priority
            first_payload
            first_left
            first_right)
          second_right))
    : Valid k v
        (ord_leq_at k d)
        (make_node
          k
          v
          (ord_leq_at k d)
          second_priority
          second_payload
          second_left
          (meld
            k
            v
            (ord_leq_at k d)
            (Node
              k
              v
              (ord_leq_at k d)
              first_rank
              first_priority
              first_payload
              first_left
              first_right)
            second_right)) =
  make_node_valid
    k
    v
    (ord_leq_at k d)
    second_priority
    second_payload
    second_left
    (meld
      k
      v
      (ord_leq_at k d)
      (Node k v (ord_leq_at k d) first_rank first_priority first_payload first_left first_right)
      second_right)
    (valid_node_left
      k
      v
      (ord_leq_at k d)
      second_rank
      second_priority
      second_payload
      second_left
      second_right
      second_valid)
    recursive_valid
    (valid_node_left_bound
      k
      v
      (ord_leq_at k d)
      second_rank
      second_priority
      second_payload
      second_left
      second_right
      second_valid)
    (all_above_root_bound
      k
      v
      (ord_leq_at k d)
      second_priority
      (meld
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          first_rank
          first_priority
          first_payload
          first_left
          first_right)
        second_right)
      (meld_all_above
        k
        v
        (ord_leq_at k d)
        second_priority
        (Node
          k
          v
          (ord_leq_at k d)
          first_rank
          first_priority
          first_payload
          first_left
          first_right)
        (valid_root_global
          k
          v
          d
          second_priority
          (Node
            k
            v
            (ord_leq_at k d)
            first_rank
            first_priority
            first_payload
            first_left
            first_right)
          first_valid
          ((proof left_false_elim for bool_or)
            (ord_leq_at k d first_priority second_priority)
            (ord_leq_at k d second_priority first_priority)
            compared
            (d.total first_priority second_priority)))
        second_right
        (all_above_node_right
          k
          v
          (ord_leq_at k d)
          second_priority
          second_rank
          second_priority
          second_payload
          second_left
          second_right
          (valid_all_above_root
            k
            v
            d
            second_rank
            second_priority
            second_payload
            second_left
            second_right
            second_valid))))

theorem meld_valid_node
      (k : Type)
      (v : Type)
      (d : Ord k)
      (first_rank : Nat)
      (first_priority : k)
      (first_payload : v)
      (first_left : PriorityQueue k v (ord_leq_at k d))
      (first_right : PriorityQueue k v (ord_leq_at k d))
      (first_valid : Valid
        k
        v
        (ord_leq_at k d)
        (Node
          k
          v
          (ord_leq_at k d)
          first_rank
          first_priority
          first_payload
          first_left
          first_right))
      (first_right_ih : (second : PriorityQueue k v (ord_leq_at k d))
        → Valid
        k
        v
        (ord_leq_at k d)
        second
        → Valid
        k
        v
        (ord_leq_at k d)
        (meld k v (ord_leq_at k d) first_right second))
      (second : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) second
      → Valid k v
        (ord_leq_at k d)
        (meld
          k
          v
          (ord_leq_at k d)
          (Node
            k
            v
            (ord_leq_at k d)
            first_rank
            first_priority
            first_payload
            first_left
            first_right)
          second) =
  match second {
    Empty ↦ λsecond_valid. first_valid;
    Node second_rank second_priority second_payload second_left second_right ↦
      λsecond_valid.
        bool_cases
          (ord_leq_at k d first_priority second_priority)
          (λchoice.
            Equal Bool (ord_leq_at k d first_priority second_priority) choice
            → Valid
              k
              v
              (ord_leq_at k d)
              (match choice {
                True ↦
                  make_node
                    k
                    v
                    (ord_leq_at k d)
                    first_priority
                    first_payload
                    first_left
                    (meld
                      k
                      v
                      (ord_leq_at k d)
                      first_right
                      (Node
                        k
                        v
                        (ord_leq_at k d)
                        second_rank
                        second_priority
                        second_payload
                        second_left
                        second_right));
                False ↦
                  make_node
                    k
                    v
                    (ord_leq_at k d)
                    second_priority
                    second_payload
                    second_left
                    (meld
                      k
                      v
                      (ord_leq_at k d)
                      (Node
                        k
                        v
                        (ord_leq_at k d)
                        first_rank
                        first_priority
                        first_payload
                        first_left
                        first_right)
                      second_right)
              }))
          (λcompared.
            meld_valid_selected_first
              k
              v
              d
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              second_rank
              second_priority
              second_payload
              second_left
              second_right
              compared
              first_valid
              second_valid
              (first_right_ih
                (Node
                  k
                  v
                  (ord_leq_at k d)
                  second_rank
                  second_priority
                  second_payload
                  second_left
                  second_right)
                second_valid))
          (λcompared.
            meld_valid_selected_second
              k
              v
              d
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              second_rank
              second_priority
              second_payload
              second_left
              second_right
              compared
              first_valid
              second_valid
              (meld_valid_node
                k
                v
                d
                first_rank
                first_priority
                first_payload
                first_left
                first_right
                first_valid
                first_right_ih
                second_right
                (valid_node_right
                  k
                  v
                  (ord_leq_at k d)
                  second_rank
                  second_priority
                  second_payload
                  second_left
                  second_right
                  second_valid)))
          Refl
  }

theorem meld_valid
      (k : Type) (v : Type) (d : Ord k) (first : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) first
      → (second : PriorityQueue k v (ord_leq_at k d))
      → Valid k v (ord_leq_at k d) second
      → Valid k v (ord_leq_at k d) (meld k v (ord_leq_at k d) first second) =
  match first {
    Empty ↦ λfirst_valid. λsecond. λsecond_valid. second_valid;
    Node first_rank first_priority first_payload first_left first_right ↦
      λfirst_valid.
        meld_valid_node
          k
          v
          d
          first_rank
          first_priority
          first_payload
          first_left
          first_right
          first_valid
          (meld_valid
            k
            v
            d
            first_right
            (valid_node_right
              k
              v
              (ord_leq_at k d)
              first_rank
              first_priority
              first_payload
              first_left
              first_right
              first_valid))
  }

theorem merge_valid
      (k : Type)
      (v : Type)
      (d : Ord k)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) left
      → Valid k v (ord_leq_at k d) right
      → Valid k v (ord_leq_at k d) (merge k v d left right) =
  λleft_valid. λright_valid. meld_valid k v d left left_valid right right_valid

theorem insert_valid
      (k : Type)
      (v : Type)
      (d : Ord k)
      (priority : k)
      (payload : v)
      (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q
      → Valid k v (ord_leq_at k d) (insert k v d priority payload q) =
  λq_valid.
    merge_valid
      k
      v
      d
      (Node
        k
        v
        (ord_leq_at k d)
        (Suc Zero)
        priority
        payload
        (Empty k v (ord_leq_at k d))
        (Empty k v (ord_leq_at k d)))
      q
      (singleton_valid k v d priority payload)
      q_valid

theorem some_not_none
      (a : Type) (item : a) (same : Equal (Option a) (Some a item) (None a))
    : Bottom =
  same

theorem pop_min_valid
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → Valid k v (ord_leq_at k d) q
      → Valid k v (ord_leq_at k d) remainder =
  match q {
    Empty ↦
      λentry.
        λremainder.
          λsame.
            λvalid.
              absurd
                (none_not_some
                  (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
                  (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder)
                  same);
    Node cached priority payload left right ↦
      λentry.
        λremainder.
          λsame.
            λvalid.
              J
                (λout _. Valid k v (ord_leq_at k d) out)
                (merge_valid
                  k
                  v
                  d
                  left
                  right
                  (valid_node_left
                    k
                    v
                    (ord_leq_at k d)
                    cached
                    priority
                    payload
                    left
                    right
                    valid)
                  (valid_node_right
                    k
                    v
                    (ord_leq_at k d)
                    cached
                    priority
                    payload
                    left
                    right
                    valid))
                (and_snd
                  (Equal (Pair k v) (mk_pair k v priority payload) entry)
                  (Equal
                    (PriorityQueue k v (ord_leq_at k d))
                    (merge k v d left right)
                    remainder)
                  same)
  }

theorem find_min_empty_equation
      (k : Type) (v : Type) (d : Ord k)
    : Equal (Option (Pair k v)) (find_min k v d (empty k v d)) (None (Pair k v)) =
  Proved

theorem find_min_node_equation
      (k : Type)
      (v : Type)
      (d : Ord k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
    : Equal
        (Option (Pair k v))
        (find_min k v d (Node k v (ord_leq_at k d) cached priority payload left right))
        (Some (Pair k v) (mk_pair k v priority payload)) =
  and_intro (Equal k priority priority) (Equal v payload payload) Refl Refl

theorem pop_min_empty_equation
      (k : Type) (v : Type) (d : Ord k)
    : Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d (empty k v d))
        (None (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))) =
  Proved

theorem pop_min_node_equation
      (k : Type)
      (v : Type)
      (d : Ord k)
      (cached : Nat)
      (priority : k)
      (payload : v)
      (left : PriorityQueue k v (ord_leq_at k d))
      (right : PriorityQueue k v (ord_leq_at k d))
    : Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d (Node k v (ord_leq_at k d) cached priority payload left right))
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair
            (Pair k v)
            (PriorityQueue k v (ord_leq_at k d))
            (mk_pair k v priority payload)
            (merge k v d left right))) =
  and_intro
    (Equal (Pair k v) (mk_pair k v priority payload) (mk_pair k v priority payload))
    (Equal
      (PriorityQueue k v (ord_leq_at k d))
      (merge k v d left right)
      (merge k v d left right))
    (and_intro (Equal k priority priority) (Equal v payload payload) Refl Refl)
    Refl

theorem find_min_global
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → Equal (Option (Pair k v)) (find_min k v d q) (Some (Pair k v) entry)
      → Valid k v (ord_leq_at k d) q
      → IsTrue (all_above k v (ord_leq_at k d) (pair_fst k v entry) q) =
  match q {
    Empty ↦ λentry. λsame. λvalid. absurd (none_not_some (Pair k v) entry same);
    Node cached priority payload left right ↦
      λentry.
        λsame.
          λvalid.
            J
              (λout_priority _.
                IsTrue
                  (all_above
                    k
                    v
                    (ord_leq_at k d)
                    out_priority
                    (Node k v (ord_leq_at k d) cached priority payload left right)))
              (valid_all_above_root k v d cached priority payload left right valid)
              (and_fst
                (Equal k priority (pair_fst k v entry))
                (Equal v payload (pair_snd k v entry))
                same)
  }

theorem pop_min_global
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → Valid k v (ord_leq_at k d) q
      → IsTrue (all_above k v (ord_leq_at k d) (pair_fst k v entry) q) =
  match q {
    Empty ↦
      λentry.
        λremainder.
          λsame.
            λvalid.
              absurd
                (none_not_some
                  (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
                  (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder)
                  same);
    Node cached priority payload left right ↦
      λentry.
        λremainder.
          λsame.
            λvalid.
              J
                (λout_priority _.
                  IsTrue
                    (all_above
                      k
                      v
                      (ord_leq_at k d)
                      out_priority
                      (Node k v (ord_leq_at k d) cached priority payload left right)))
                (valid_all_above_root k v d cached priority payload left right valid)
                (and_fst
                  (Equal k priority (pair_fst k v entry))
                  (Equal v payload (pair_snd k v entry))
                  (and_fst
                    (Equal (Pair k v) (mk_pair k v priority payload) entry)
                    (Equal
                      (PriorityQueue k v (ord_leq_at k d))
                      (merge k v d left right)
                      remainder)
                    same))
  }

fn pop_is_none
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Prop =
  Equal
    (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
    (pop_min k v d q)
    (None (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))

fn queue_is_empty
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Prop =
  Equal (PriorityQueue k v (ord_leq_at k d)) q (Empty k v (ord_leq_at k d))

theorem pop_none_implies_empty
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : pop_is_none k v d q → queue_is_empty k v d q =
  match q {
    Empty ↦ λsame. Proved;
    Node cached priority payload left right ↦ λsame. absurd same
  }

theorem empty_implies_pop_none
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : queue_is_empty k v d q → pop_is_none k v d q =
  match q {
    Empty ↦ λsame. Proved;
    Node cached priority payload left right ↦ λsame. absurd same
  }

theorem pop_none_iff_empty
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : And
        (pop_is_none k v d q → queue_is_empty k v d q)
        (queue_is_empty k v d q → pop_is_none k v d q) =
  and_intro
    (pop_is_none k v d q → queue_is_empty k v d q)
    (queue_is_empty k v d q → pop_is_none k v d q)
    (pop_none_implies_empty k v d q)
    (empty_implies_pop_none k v d q)

theorem add_one_left (n : Nat) : Equal Nat (add (Suc Zero) n) (Suc n) =
  trans
    Nat
    (add (Suc Zero) n)
    (Suc (add Zero n))
    (Suc n)
    ((proof suc_l for add) Zero n)
    ((proof zero_l for add) n)

theorem size_zero_implies_empty
      (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq)
    : Equal Nat (size k v leq q) Zero → Equal (PriorityQueue k v leq) q (Empty k v leq) =
  match q {
    Empty ↦ λsame. Proved;
    Node cached priority payload left right ↦
      λsame.
        absurd
          (trans
            Nat
            (Suc
              (add
                (count_by k v leq (λpriority. λpayload. True) left)
                (count_by k v leq (λpriority. λpayload. True) right)))
            (add
              (Suc Zero)
              (add
                (count_by k v leq (λpriority. λpayload. True) left)
                (count_by k v leq (λpriority. λpayload. True) right)))
            Zero
            (sym
              Nat
              (add
                (Suc Zero)
                (add
                  (count_by k v leq (λpriority. λpayload. True) left)
                  (count_by k v leq (λpriority. λpayload. True) right)))
              (Suc
                (add
                  (count_by k v leq (λpriority. λpayload. True) left)
                  (count_by k v leq (λpriority. λpayload. True) right)))
              (add_one_left
                (add
                  (count_by k v leq (λpriority. λpayload. True) left)
                  (count_by k v leq (λpriority. λpayload. True) right))))
            same)
  }

theorem empty_implies_size_zero
      (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq)
    : Equal (PriorityQueue k v leq) q (Empty k v leq) → Equal Nat (size k v leq q) Zero =
  match q {
    Empty ↦ λsame. Proved;
    Node cached priority payload left right ↦ λsame. absurd same
  }

theorem size_zero_iff_empty
      (k : Type) (v : Type) (leq : k → k → Bool) (q : PriorityQueue k v leq)
    : And
        (Equal Nat (size k v leq q) Zero → Equal (PriorityQueue k v leq) q (Empty k v leq))
        (Equal (PriorityQueue k v leq) q (Empty k v leq) → Equal Nat (size k v leq q) Zero) =
  and_intro
    (Equal Nat (size k v leq q) Zero → Equal (PriorityQueue k v leq) q (Empty k v leq))
    (Equal (PriorityQueue k v leq) q (Empty k v leq) → Equal Nat (size k v leq q) Zero)
    (size_zero_implies_empty k v leq q)
    (empty_implies_size_zero k v leq q)

theorem pop_min_size_step
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → Equal Nat (size k v (ord_leq_at k d) q) (Suc (size k v (ord_leq_at k d) remainder)) =
  λentry.
    λremainder.
      λsame.
        trans
          Nat
          (size k v (ord_leq_at k d) q)
          (add (Suc Zero) (size k v (ord_leq_at k d) remainder))
          (Suc (size k v (ord_leq_at k d) remainder))
          (pop_min_count k v d (λpriority. λpayload. True) q entry remainder same)
          (add_one_left (size k v (ord_leq_at k d) remainder))

fn list_count_by (k : Type) (v : Type) (p : k → v → Bool) (entries : List (Pair k v)) : Nat =
  match entries {
    Nil ↦ Zero;
    Cons entry rest ↦
      add
        (contribution (p (pair_fst k v entry) (pair_snd k v entry)))
        (list_count_by k v p rest)
  }

fn entries_all_above
      (k : Type) (v : Type) (d : Ord k) (bound : k) (entries : List (Pair k v))
    : Bool =
  match entries {
    Nil ↦ True;
    Cons entry rest ↦
      bool_and (ord_leq_at k d bound (pair_fst k v entry)) (entries_all_above k v d bound rest)
  }

fn nondecreasing (k : Type) (v : Type) (d : Ord k) (entries : List (Pair k v)) : Bool =
  match entries {
    Nil ↦ True;
    Cons entry rest ↦
      bool_and (entries_all_above k v d (pair_fst k v entry) rest) (nondecreasing k v d rest)
  }

fn observed_entries
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : List (Pair k v) =
  pair_fst (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) (observe_pops k v d n q)

fn observed_remainder
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : PriorityQueue k v (ord_leq_at k d) =
  pair_snd (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) (observe_pops k v d n q)

theorem pop_min_all_above
      (k : Type) (v : Type) (d : Ord k) (bound : k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → IsTrue (all_above k v (ord_leq_at k d) bound q)
      → IsTrue (all_above k v (ord_leq_at k d) bound remainder) =
  match q {
    Empty ↦
      λentry.
        λremainder.
          λsame.
            λabove.
              absurd
                (none_not_some
                  (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
                  (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder)
                  same);
    Node cached priority payload left right ↦
      λentry.
        λremainder.
          λsame.
            λabove.
              J
                (λout _. IsTrue (all_above k v (ord_leq_at k d) bound out))
                (meld_all_above
                  k
                  v
                  (ord_leq_at k d)
                  bound
                  left
                  (all_above_node_left
                    k
                    v
                    (ord_leq_at k d)
                    bound
                    cached
                    priority
                    payload
                    left
                    right
                    above)
                  right
                  (all_above_node_right
                    k
                    v
                    (ord_leq_at k d)
                    bound
                    cached
                    priority
                    payload
                    left
                    right
                    above))
                (and_snd
                  (Equal (Pair k v) (mk_pair k v priority payload) entry)
                  (Equal
                    (PriorityQueue k v (ord_leq_at k d))
                    (merge k v d left right)
                    remainder)
                  same)
  }

theorem pop_min_entry_bound
      (k : Type) (v : Type) (d : Ord k) (bound : k) (q : PriorityQueue k v (ord_leq_at k d))
    : (entry : Pair k v)
      → (remainder : PriorityQueue k v (ord_leq_at k d))
      → Equal
        (Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d))))
        (pop_min k v d q)
        (Some
          (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
          (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder))
      → IsTrue (all_above k v (ord_leq_at k d) bound q)
      → IsTrue (ord_leq_at k d bound (pair_fst k v entry)) =
  match q {
    Empty ↦
      λentry.
        λremainder.
          λsame.
            λabove.
              absurd
                (none_not_some
                  (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))
                  (mk_pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)) entry remainder)
                  same);
    Node cached priority payload left right ↦
      λentry.
        λremainder.
          λsame.
            λabove.
              J
                (λout_priority _. IsTrue (ord_leq_at k d bound out_priority))
                (all_above_node_root
                  k
                  v
                  (ord_leq_at k d)
                  bound
                  cached
                  priority
                  payload
                  left
                  right
                  above)
                (and_fst
                  (Equal k priority (pair_fst k v entry))
                  (Equal v payload (pair_snd k v entry))
                  (and_fst
                    (Equal (Pair k v) (mk_pair k v priority payload) entry)
                    (Equal
                      (PriorityQueue k v (ord_leq_at k d))
                      (merge k v d left right)
                      remainder)
                    same))
  }

theorem observe_valid
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q → Valid k v (ord_leq_at k d) (observed_remainder k v d n q) =
  match n {
    Zero ↦ λvalid. valid;
    Suc remaining ↦
      match q {
        Empty ↦ λvalid. valid;
        Node cached priority payload left right ↦
          λvalid.
            observe_valid
              k
              v
              d
              remaining
              (merge k v d left right)
              (pop_min_valid
                k
                v
                d
                (Node k v (ord_leq_at k d) cached priority payload left right)
                (mk_pair k v priority payload)
                (merge k v d left right)
                Refl
                valid)
      }
  }

theorem observe_remainder_above
      (k : Type)
      (v : Type)
      (d : Ord k)
      (bound : k)
      (n : Nat)
      (q : PriorityQueue k v (ord_leq_at k d))
    : IsTrue (all_above k v (ord_leq_at k d) bound q)
      → IsTrue (all_above k v (ord_leq_at k d) bound (observed_remainder k v d n q)) =
  match n {
    Zero ↦ λabove. above;
    Suc remaining ↦
      match q {
        Empty ↦ λabove. above;
        Node cached priority payload left right ↦
          λabove.
            observe_remainder_above
              k
              v
              d
              bound
              remaining
              (merge k v d left right)
              (pop_min_all_above
                k
                v
                d
                bound
                (Node k v (ord_leq_at k d) cached priority payload left right)
                (mk_pair k v priority payload)
                (merge k v d left right)
                Refl
                above)
      }
  }

theorem observe_entries_above
      (k : Type)
      (v : Type)
      (d : Ord k)
      (bound : k)
      (n : Nat)
      (q : PriorityQueue k v (ord_leq_at k d))
    : IsTrue (all_above k v (ord_leq_at k d) bound q)
      → IsTrue (entries_all_above k v d bound (observed_entries k v d n q)) =
  match n {
    Zero ↦ λabove. Proved;
    Suc remaining ↦
      match q {
        Empty ↦ λabove. Proved;
        Node cached priority payload left right ↦
          λabove.
            (proof intro for bool_and)
              (ord_leq_at k d bound priority)
              (entries_all_above
                k
                v
                d
                bound
                (observed_entries k v d remaining (merge k v d left right)))
              (pop_min_entry_bound
                k
                v
                d
                bound
                (Node k v (ord_leq_at k d) cached priority payload left right)
                (mk_pair k v priority payload)
                (merge k v d left right)
                Refl
                above)
              (observe_entries_above
                k
                v
                d
                bound
                remaining
                (merge k v d left right)
                (pop_min_all_above
                  k
                  v
                  d
                  bound
                  (Node k v (ord_leq_at k d) cached priority payload left right)
                  (mk_pair k v priority payload)
                  (merge k v d left right)
                  Refl
                  above))
      }
  }

theorem observe_nondecreasing
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q → IsTrue (nondecreasing k v d (observed_entries k v d n q)) =
  match n {
    Zero ↦ λvalid. Proved;
    Suc remaining ↦
      match q {
        Empty ↦ λvalid. Proved;
        Node cached priority payload left right ↦
          λvalid.
            (proof intro for bool_and)
              (entries_all_above
                k
                v
                d
                priority
                (observed_entries k v d remaining (merge k v d left right)))
              (nondecreasing k v d (observed_entries k v d remaining (merge k v d left right)))
              (observe_entries_above
                k
                v
                d
                priority
                remaining
                (merge k v d left right)
                (pop_min_all_above
                  k
                  v
                  d
                  priority
                  (Node k v (ord_leq_at k d) cached priority payload left right)
                  (mk_pair k v priority payload)
                  (merge k v d left right)
                  Refl
                  (pop_min_global
                    k
                    v
                    d
                    (Node k v (ord_leq_at k d) cached priority payload left right)
                    (mk_pair k v priority payload)
                    (merge k v d left right)
                    Refl
                    valid)))
              (observe_nondecreasing
                k
                v
                d
                remaining
                (merge k v d left right)
                (pop_min_valid
                  k
                  v
                  d
                  (Node k v (ord_leq_at k d) cached priority payload left right)
                  (mk_pair k v priority payload)
                  (merge k v d left right)
                  Refl
                  valid))
      }
  }

theorem observe_count
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (n : Nat)
      (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (count_by k v (ord_leq_at k d) p q)
        (add
          (list_count_by k v p (observed_entries k v d n q))
          (count_by k v (ord_leq_at k d) p (observed_remainder k v d n q))) =
  match n {
    Zero ↦
      sym
        Nat
        (add Zero (count_by k v (ord_leq_at k d) p q))
        (count_by k v (ord_leq_at k d) p q)
        ((proof zero_l for add) (count_by k v (ord_leq_at k d) p q));
    Suc remaining ↦
      match q {
        Empty ↦ sym Nat (add Zero Zero) Zero ((proof zero_l for add) Zero);
        Node cached priority payload left right ↦
          let
            entry = mk_pair k v priority payload;
            next = merge k v d left right;
            item_count = contribution (p priority payload);
            tail_entries = observed_entries k v d remaining next;
            tail_remainder = observed_remainder k v d remaining next;
            next_count = count_by k v (ord_leq_at k d) p next;
            tail_list_count = list_count_by k v p tail_entries;
            tail_remainder_count = count_by k v (ord_leq_at k d) p tail_remainder
          in
            trans
              Nat
              (count_by
                k
                v
                (ord_leq_at k d)
                p
                (Node k v (ord_leq_at k d) cached priority payload left right))
              (add item_count next_count)
              (add (add item_count tail_list_count) tail_remainder_count)
              (pop_min_count
                k
                v
                d
                p
                (Node k v (ord_leq_at k d) cached priority payload left right)
                (mk_pair k v priority payload)
                (merge k v d left right)
                Refl)
              (trans
                Nat
                (add item_count next_count)
                (add item_count (add tail_list_count tail_remainder_count))
                (add (add item_count tail_list_count) tail_remainder_count)
                (cong
                  Nat
                  Nat
                  next_count
                  (add tail_list_count tail_remainder_count)
                  (λvalue. add item_count value)
                  (observe_count k v d p remaining next))
                ((proof assoc for add) item_count tail_list_count tail_remainder_count))
      }
  }

theorem queue_cases
      (k : Type)
      (v : Type)
      (leq : k → k → Bool)
      (q : PriorityQueue k v leq)
      (motive : PriorityQueue k v leq → Prop)
      (empty_case : motive (Empty k v leq))
      (node_case : (cached : Nat)
        → (priority : k)
        → (payload : v)
        → (left : PriorityQueue k v leq)
        → (right : PriorityQueue k v leq)
        → motive
        (Node k v leq cached priority payload left right))
    : motive q =
  match q {
    Empty ↦ empty_case;
    Node cached priority payload left right ↦ node_case cached priority payload left right
  }

theorem observe_length
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (length (Pair k v) (observed_entries k v d n q))
        (nat_min n (size k v (ord_leq_at k d) q)) =
  match n {
    Zero ↦ ((proof zero_l for add) Zero);
    Suc remaining ↦
      queue_cases
        k
        v
        (ord_leq_at k d)
        q
        (λqueue.
          Equal
            Nat
            (length (Pair k v) (observed_entries k v d (Suc remaining) queue))
            (nat_min (Suc remaining) (size k v (ord_leq_at k d) queue)))
        ((proof zero_l for add) Zero)
        (λcached.
          λpriority.
            λpayload.
              λleft.
                λright.
                  let
                    next = merge k v d left right;
                    tail_length = length (Pair k v) (observed_entries k v d remaining next);
                    next_size = size k v (ord_leq_at k d) next;
                    queue_size =
                      size
                        k
                        v
                        (ord_leq_at k d)
                        (Node k v (ord_leq_at k d) cached priority payload left right)
                  in
                    trans
                      Nat
                      (Suc tail_length)
                      (Suc (nat_min remaining next_size))
                      (nat_min (Suc remaining) queue_size)
                      (cong
                        Nat
                        Nat
                        tail_length
                        (nat_min remaining next_size)
                        Suc
                        (observe_length k v d remaining next))
                      (sym
                        Nat
                        (nat_min (Suc remaining) queue_size)
                        (nat_min (Suc remaining) (Suc next_size))
                        (cong
                          Nat
                          Nat
                          queue_size
                          (Suc next_size)
                          (λamount. nat_min (Suc remaining) amount)
                          (pop_min_size_step
                            k
                            v
                            d
                            (Node k v (ord_leq_at k d) cached priority payload left right)
                            (mk_pair k v priority payload)
                            (merge k v d left right)
                            (pop_min_node_equation k v d cached priority payload left right)))))
  }

theorem nat_min_self (n : Nat) : Equal Nat (nat_min n n) n =
  match n {
    Zero ↦ Proved;
    Suc previous ↦ cong Nat Nat (nat_min previous previous) previous Suc (nat_min_self previous)
  }

theorem nat_sub_self (n : Nat) : Equal Nat (nat_sub n n) Zero =
  match n {
    Zero ↦ Proved;
    Suc previous ↦ nat_sub_self previous
  }

theorem equal_self (a : Type) (value : a) : Equal a value value = Refl

theorem observe_remainder_zero
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Equal (PriorityQueue k v (ord_leq_at k d)) (observed_remainder k v d Zero q) q =
  equal_self (PriorityQueue k v (ord_leq_at k d)) q

theorem observe_remainder_size
      (k : Type) (v : Type) (d : Ord k) (n : Nat) (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (size k v (ord_leq_at k d) (observed_remainder k v d n q))
        (nat_sub (size k v (ord_leq_at k d) q) n) =
  match n {
    Zero ↦
      cong
        (PriorityQueue k v (ord_leq_at k d))
        Nat
        (observed_remainder k v d Zero q)
        q
        (size k v (ord_leq_at k d))
        (observe_remainder_zero k v d q);
    Suc remaining ↦
      queue_cases
        k
        v
        (ord_leq_at k d)
        q
        (λqueue.
          Equal
            Nat
            (size k v (ord_leq_at k d) (observed_remainder k v d (Suc remaining) queue))
            (nat_sub (size k v (ord_leq_at k d) queue) (Suc remaining)))
        (nat_sub_self Zero)
        (λcached.
          λpriority.
            λpayload.
              λleft.
                λright.
                  let
                    next = merge k v d left right;
                    next_size = size k v (ord_leq_at k d) next;
                    queue_size =
                      size
                        k
                        v
                        (ord_leq_at k d)
                        (Node k v (ord_leq_at k d) cached priority payload left right)
                  in
                    trans
                      Nat
                      (size k v (ord_leq_at k d) (observed_remainder k v d remaining next))
                      (nat_sub next_size remaining)
                      (nat_sub queue_size (Suc remaining))
                      (observe_remainder_size k v d remaining next)
                      (sym
                        Nat
                        (nat_sub queue_size (Suc remaining))
                        (nat_sub (Suc next_size) (Suc remaining))
                        (cong
                          Nat
                          Nat
                          queue_size
                          (Suc next_size)
                          (λamount. nat_sub amount (Suc remaining))
                          (pop_min_size_step
                            k
                            v
                            d
                            (Node k v (ord_leq_at k d) cached priority payload left right)
                            (mk_pair k v priority payload)
                            (merge k v d left right)
                            (pop_min_node_equation k v d cached priority payload left right)))))
  }

theorem drain_remainder_empty
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Equal
        (PriorityQueue k v (ord_leq_at k d))
        (observed_remainder k v d (size k v (ord_leq_at k d) q) q)
        (Empty k v (ord_leq_at k d)) =
  let
    remainder = observed_remainder k v d (size k v (ord_leq_at k d) q) q;
    queue_size = size k v (ord_leq_at k d) q
  in
    size_zero_implies_empty
      k
      v
      (ord_leq_at k d)
      remainder
      (trans
        Nat
        (size k v (ord_leq_at k d) remainder)
        (nat_sub queue_size queue_size)
        Zero
        (observe_remainder_size k v d queue_size q)
        (nat_sub_self queue_size))

theorem drain_length
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (length (Pair k v) (observed_entries k v d (size k v (ord_leq_at k d) q) q))
        (size k v (ord_leq_at k d) q) =
  let queue_size =
    size k v (ord_leq_at k d) q
  in
    trans
      Nat
      (length (Pair k v) (observed_entries k v d queue_size q))
      (nat_min queue_size queue_size)
      queue_size
      (observe_length k v d queue_size q)
      (nat_min_self queue_size)

theorem drain_count
      (k : Type)
      (v : Type)
      (d : Ord k)
      (p : k → v → Bool)
      (q : PriorityQueue k v (ord_leq_at k d))
    : Equal Nat
        (count_by k v (ord_leq_at k d) p q)
        (list_count_by k v p (observed_entries k v d (size k v (ord_leq_at k d) q) q)) =
  let
    queue_size = size k v (ord_leq_at k d) q;
    entries = observed_entries k v d queue_size q;
    remainder = observed_remainder k v d queue_size q;
    entry_count = list_count_by k v p entries;
    remainder_count = count_by k v (ord_leq_at k d) p remainder
  in
    trans
      Nat
      (count_by k v (ord_leq_at k d) p q)
      (add entry_count remainder_count)
      entry_count
      (observe_count k v d p queue_size q)
      (trans
        Nat
        (add entry_count remainder_count)
        (add entry_count Zero)
        entry_count
        (cong
          Nat
          Nat
          remainder_count
          Zero
          (λamount. add entry_count amount)
          (cong
            (PriorityQueue k v (ord_leq_at k d))
            Nat
            remainder
            (Empty k v (ord_leq_at k d))
            (count_by k v (ord_leq_at k d) p)
            (drain_remainder_empty k v d q)))
        (equal_self Nat entry_count))

theorem drain_valid
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q
      → Valid k v (ord_leq_at k d) (observed_remainder k v d (size k v (ord_leq_at k d) q) q) =
  observe_valid k v d (size k v (ord_leq_at k d) q) q

theorem drain_nondecreasing
      (k : Type) (v : Type) (d : Ord k) (q : PriorityQueue k v (ord_leq_at k d))
    : Valid k v (ord_leq_at k d) q
      → IsTrue (nondecreasing k v d (observed_entries k v d (size k v (ord_leq_at k d) q) q)) =
  observe_nondecreasing k v d (size k v (ord_leq_at k d) q) q

theorem nontrivial_validity_witness
      (d : Ord Nat)
    : Valid Nat Nat
        (ord_leq_at Nat d)
        (insert
          Nat
          Nat
          d
          (Suc Zero)
          (Suc (Suc (Suc Zero)))
          (insert
            Nat
            Nat
            d
            Zero
            (Suc (Suc Zero))
            (insert Nat Nat d (Suc (Suc Zero)) (Suc Zero) (empty Nat Nat d)))) =
  insert_valid
    Nat
    Nat
    d
    (Suc Zero)
    (Suc (Suc (Suc Zero)))
    (insert
      Nat
      Nat
      d
      Zero
      (Suc (Suc Zero))
      (insert Nat Nat d (Suc (Suc Zero)) (Suc Zero) (empty Nat Nat d)))
    (insert_valid
      Nat
      Nat
      d
      Zero
      (Suc (Suc Zero))
      (insert Nat Nat d (Suc (Suc Zero)) (Suc Zero) (empty Nat Nat d))
      (insert_valid
        Nat
        Nat
        d
        (Suc (Suc Zero))
        (Suc Zero)
        (empty Nat Nat d)
        (empty_valid Nat Nat d)))

const malformed_cache_witness : PriorityQueue Nat Nat leq_nat =
  Node Nat Nat leq_nat Zero Zero Zero (Empty Nat Nat leq_nat) (Empty Nat Nat leq_nat)

theorem malformed_cache_refuted
    : Equal Bool (valid_bool Nat Nat leq_nat malformed_cache_witness) False =
  Proved

fn valid_leaf_witness (priority : Nat) (payload : Nat) : PriorityQueue Nat Nat leq_nat =
  Node
    Nat
    Nat
    leq_nat
    (Suc Zero)
    priority
    payload
    (Empty Nat Nat leq_nat)
    (Empty Nat Nat leq_nat)

const malformed_balance_witness : PriorityQueue Nat Nat leq_nat =
  Node
    Nat
    Nat
    leq_nat
    (Suc (Suc Zero))
    Zero
    Zero
    (Empty Nat Nat leq_nat)
    (valid_leaf_witness Zero (Suc Zero))

theorem malformed_balance_refuted
    : Equal Bool (valid_bool Nat Nat leq_nat malformed_balance_witness) False =
  Proved

const malformed_heap_order_witness : PriorityQueue Nat Nat leq_nat =
  Node
    Nat
    Nat
    leq_nat
    (Suc Zero)
    (Suc Zero)
    Zero
    (valid_leaf_witness Zero (Suc Zero))
    (Empty Nat Nat leq_nat)

theorem malformed_heap_order_refuted
    : Equal Bool (valid_bool Nat Nat leq_nat malformed_heap_order_witness) False =
  Proved
```

## Using it

Clients choose a lawful priority order explicitly. Each operation returns a new
value, so both inputs to a merge and the original queue before a pop remain
available. `find_min` and `pop_min` return `None` for an empty queue; otherwise
they expose the root entry, and `pop_min` also returns the queue formed by
melding the root's children.

Equal priorities deliberately carry no FIFO promise. A client that needs stable
ties can include its own sequence number in the priority type and supply the
corresponding lawful order.

## Laws and proofs

The public operations implement the behavioral equations in
`spec/50-stdlib/58a-priority-queues.md`: insertion adds one occurrence, merge
adds both multisets, and a successful pop removes exactly the returned minimum
occurrence. The private proof suite establishes these equations for every
predicate, proves that `meld`, `insert`, and successful `pop_min` preserve the
leftist invariant, and proves that an actual root bounds every descendant under
the supplied lawful order.

The private `observe_pops` proof observation calls the actual `pop_min`. For
every finite prefix it preserves validity and predicate counts, returns a
nondecreasing list, and has length `min n (size q)`. At `n = size q`, the
remainder is empty, the list has exactly `size q` entries, and every predicate
count is conserved. The observer and every theorem remain private, so the
six-name public interface is unchanged. The acceptance suite independently
retains its finite computational, privacy, persistence, and structural-cost
evidence.

## Design notes

The leftist invariant makes meld's descent local and auditable. For valid
operands, every recursive two-nonempty meld call advances along one current
right spine. It makes one priority comparison, and the terminal empty call makes
none. `make_node` reads only the two child ranks and reconstructs one node; it
does not traverse either child.

This structural account does not make comparison of an arbitrary priority or a
unary `Nat` constant-time, and it is not a native wall-clock claim. There is no
whole-tree sizing pass, conversion to a sorted list, fuel wrapper, or alternate
extraction engine.

## References

- Chris Okasaki, *Purely Functional Data Structures*, Cambridge University
  Press, 1998 — develops persistent heaps and the leftist rank invariant.
- [Leftist tree](https://en.wikipedia.org/wiki/Leftist_tree) — orientation to
  the meld-first representation and its right-spine property.

## Trust and derivation

The package implements `Data.Collections.PriorityQueue` and follows
`spec/50-stdlib/58a-priority-queues.md`. Its public API is exactly
`PriorityQueue`, `empty`, `insert`, `find_min`, `pop_min`, and `merge`.

The carrier is an ordinary checked inductive in `Type`. The implementation uses
the compiler-origin `Nat`, `Bool`, `Option`, and `Pair` carriers and imports the
canonical `Ord`, `ord_leq_at`, and `leq_nat` declarations from
`Core.Classes.LawfulClasses`. It adds no axiom, postulate, primitive, foreign
declaration, proof-relevant `Omega` carrier, or trusted-base entry. Loading the
provider closure retains that closure's existing, separately disclosed
assumptions without duplicating them.

`conformance/stdlib/collections/seed-priority-queue.md` defines the public,
multiset, order, persistence, private-validity, structural-charge, and mutation
observations used to validate the computational implementation independently.
The provider's private checked suite supplies the general validity,
multiplicity, minimum, and total-drain proofs. Machine-checked complexity
remains a separate residual.
