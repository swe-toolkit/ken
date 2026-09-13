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
import Core.Classes.LawfulClasses (Ord, ord_leq_at, leq_nat)

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
occurrence. The accompanying acceptance suite checks those equations on fixed
examples and bounded exhaustive traces, and independently checks the selected
leftist representation on every produced node.

These results are **tested computation**, not general kernel proofs. General
multiplicity conservation, minimum extraction, validity preservation, and
nondecreasing drain proofs remain deferred. No public result type carries an
unproved validity or conservation obligation.

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
observations used to validate this computational implementation. General law
proofs and machine-checked complexity remain separate residuals.
