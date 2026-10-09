# Finite certificates for enumerated carriers

A finite certificate lists values of a carrier and proves that every value
occurs in that list. Duplicates are permitted, and neither membership nor
coverage requires decidable equality. `unit_finite` and `bool_finite` certify
the built-in carriers, `fin_finite` certifies bounded indices, and
`pair_finite` composes certificates for paired state carriers.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Enumeration is separate evidence, not a property of an arbitrary state type.
`Finite q` makes both the list and its coverage proof explicit. A machine or
another algorithm may choose an enumeration without requiring `DecEq q`.

## 2. Definition

The constructor stores a list and a coverage proof in Ω. The two constructors
reuse Vector's bounded indices and Derived's general list membership tools.

```ken
import Data.Vector.Vector (Fin, FZero, FSuc)

import Data.Collections.Derived
  (list_elem,
    list_elem_head,
    list_elem_later,
    list_elem_map,
    list_elem_concat_map,
    list_elem_transport,
    map,
    concat_map)

pub data Finite (q : Type) : Type where {
  MkFinite : (listed : List q) → ((x : q) → list_elem q x listed) → Finite q
}

export MkFinite

pub fn elements (q : Type) (f : Finite q) : List q =
  match f {
    MkFinite listed covered ↦ listed
  }

pub theorem covers (q : Type) (fq : Finite q) : (x : q) → list_elem q x (elements q fq) =
  match fq {
    MkFinite listed covered ↦ covered
  }

pub fn fin_finite (n : Nat) : Finite (Fin n) =
  MkFinite (Fin n) (fin_elements n) (fin_elements_cover n)

pub const unit_finite : Finite Unit =
  MkFinite
    Unit
    (Cons Unit MkUnit (Nil Unit))
    (λu.
      match u {
        MkUnit ↦ list_elem_head Unit MkUnit (Nil Unit)
      })

pub const bool_finite : Finite Bool =
  MkFinite
    Bool
    (Cons Bool True (Cons Bool False (Nil Bool)))
    (λb.
      match b {
        True ↦ list_elem_head Bool True (Cons Bool False (Nil Bool));
        False ↦
          list_elem_later
            Bool
            False
            True
            (Cons Bool False (Nil Bool))
            (list_elem_head Bool False (Nil Bool))
      })

pub fn fin_elements (n : Nat) : List (Fin n) =
  match n {
    Zero ↦ Nil (Fin Zero);
    Suc m ↦ Cons (Fin (Suc m)) (FZero m) (map (Fin m) (Fin (Suc m)) (FSuc m) (fin_elements m))
  }

pub theorem fin_elements_cover (n : Nat) (i : Fin n) : list_elem (Fin n) i (fin_elements n) =
  match i {
    FZero m ↦
      list_elem_head
        (Fin (Suc m))
        (FZero m)
        (map (Fin m) (Fin (Suc m)) (FSuc m) (fin_elements m));
    FSuc m j ↦
      list_elem_later
        (Fin (Suc m))
        (FSuc m j)
        (FZero m)
        (map (Fin m) (Fin (Suc m)) (FSuc m) (fin_elements m))
        (list_elem_map
          (Fin m)
          (Fin (Suc m))
          (FSuc m)
          j
          (fin_elements m)
          (fin_elements_cover m j))
  }

pub fn pair_finite (q : Type) (r : Type) (fq : Finite q) (fr : Finite r) : Finite (Pair q r) =
  MkFinite (Pair q r) (pair_elements q r (elements q fq) (elements r fr)) (pair_cover q r fq fr)

fn pair_elements (q : Type) (r : Type) (xs : List q) (ys : List r) : List (Pair q r) =
  concat_map q (Pair q r) (pair_row q r ys) xs

fn pair_row (q : Type) (r : Type) (ys : List r) (x : q) : List (Pair q r) =
  map r (Pair q r) (mk_pair q r x) ys

theorem pair_cover
      (q : Type) (r : Type) (fq : Finite q) (fr : Finite r) (p : Pair q r)
    : list_elem (Pair q r) p (pair_elements q r (elements q fq) (elements r fr)) =
  list_elem_transport
    (Pair q r)
    (mk_pair q r (pair_fst q r p) (pair_snd q r p))
    p
    (pair_eta q r p)
    (pair_elements q r (elements q fq) (elements r fr))
    (pair_cover_at q r fq fr (pair_fst q r p) (pair_snd q r p))

theorem pair_cover_at
      (q : Type) (r : Type) (fq : Finite q) (fr : Finite r) (x : q) (y : r)
    : list_elem
        (Pair q r)
        (mk_pair q r x y)
        (pair_elements q r (elements q fq) (elements r fr)) =
  list_elem_concat_map
    q
    (Pair q r)
    (pair_row q r (elements r fr))
    (mk_pair q r x y)
    x
    (elements q fq)
    (covers q fq x)
    (list_elem_map r (Pair q r) (mk_pair q r x) y (elements r fr) (covers r fr y))

theorem pair_eta
      (q : Type) (r : Type) (p : Pair q r)
    : Equal (Pair q r) (mk_pair q r (pair_fst q r p) (pair_snd q r p)) p =
  Refl
```

## 3. Using it

`unit_finite` covers the sole value of `Unit`; `bool_finite` lists both Boolean
values. `fin_finite n` certifies `Fin n`, including the empty carrier
`Fin Zero`. For product or intersection automata, `pair_finite` combines
certificates for the two component state types.

```ken example
const one_index : Finite (Fin (Suc Zero)) = fin_finite (Suc Zero)

const two_indices : Finite (Pair (Fin (Suc Zero)) (Fin (Suc Zero))) =
  pair_finite (Fin (Suc Zero)) (Fin (Suc Zero)) one_index one_index

const bool_and_unit : Finite (Pair Bool Unit) = pair_finite Bool Unit bool_finite unit_finite
```

## 4. Laws & proofs

`covers` projects the checked evidence stored in the certificate.
`unit_finite` covers `MkUnit` by head membership; `bool_finite` covers both
constructors by head or later membership. `fin_elements_cover` proceeds by
induction on bounded indices, lifting membership through a mapped successor
list. `pair_cover` uses membership of each component and transports through
pair η; it does not need a pair equality decision.

## 5. Design notes

The list may repeat values; the certificate promises coverage, not uniqueness
or a time bound. Truncated membership eliminates only into propositions.
The private pair enumeration and coverage lemmas are implementation details.

## 6. References

- [Finite set](https://en.wikipedia.org/wiki/Finite_set) — Wikipedia;
  orientation to finite carriers without prescribing an enumeration order.
- *Introduction to Automata Theory, Languages, and Computation* — Hopcroft,
  Motwani, and Ullman; context for finite-state automata and their products.

## 7. Trust & derivation

The contract is
[formal languages §2](../../../../spec/50-stdlib/61-formal-languages.md).
The public surface comprises `Finite`, `MkFinite`, `elements`, `covers`,
`fin_finite`, `unit_finite`, `bool_finite`, `fin_elements`,
`fin_elements_cover`, and `pair_finite`.

| Reader task | Section |
|---|---|
| Obtain or compose a certificate | [Definition](#2-definition), [Using it](#3-using-it) |
| Inspect coverage | [Laws & proofs](#4-laws--proofs) |
| Review the proof and trust boundary | [Design notes](#5-design-notes) |

The derivation is ordinary checked `List`, `Fin`, `Pair`, and truncation, with
public imports from Vector and Derived. The component coverage is inherited
from its input certificates. It adds no primitive, Axiom, or local
`trusted_base()` entry; inherited assumptions from a provider are not erased.
Finite-state reachability consumes these certificates; NFA subset
construction reuses the Unit and Bool certificates for its mask carrier.
