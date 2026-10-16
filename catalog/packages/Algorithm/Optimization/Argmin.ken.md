# `Argmin` — earliest least-cost candidate

`argmin` selects an existing candidate with minimum natural-number cost,
retaining the earliest candidate when costs tie.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

A nonempty finite input always has a least-cost candidate without requiring
an equality operation on its values. Strictly comparing each tail candidate
against the current head retains the head whenever two costs are equal.
The input's existing `NonEmpty` carrier supplies the head and ordered tail.

## Definition

The operation and its seven public laws lead. Private selection helpers and
inductive proofs follow; two generic Nat-order facts come from their canonical
owner, `Data.Numeric.Nat.Order`.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)

import Core.Logic.Or (Or, Inl, Inr)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived (list_elem, list_elem_head, list_elem_later)

import Data.Collections.NonEmpty
  (NonEmpty,
    nonempty_cons,
    nonempty_singleton,
    nonempty_head,
    nonempty_tail,
    nonempty_to_list,
    nonempty_to_list_shape)

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

fn argmin_from (a : Type) (cost : a → Nat) (x : a) (rest : List a) : a =
  match rest {
    Nil ↦ x;
    Cons y ys ↦ cheaper a cost x (argmin_from a cost y ys)
  }

fn cheaper (a : Type) (cost : a → Nat) (incumbent : a) (challenger : a) : a =
  choose a (lt_nat (cost challenger) (cost incumbent)) challenger incumbent

fn choose (a : Type) (b : Bool) (yes : a) (no : a) : a =
  match b {
    True ↦ yes;
    False ↦ no
  }

theorem argmin_from_member
      (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    : list_elem a (argmin_from a cost x rest) (Cons a x rest) =
  match rest {
    Nil ↦ list_elem_head a x (Nil a);
    Cons y ys ↦
      choose_elem
        a
        (lt_nat (cost (argmin_from a cost y ys)) (cost x))
        (argmin_from a cost y ys)
        x
        (Cons a y ys)
        (argmin_from_member a cost y ys)
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
```

## Using it

Call `argmin` with the value type, a natural-number cost function, and an
existing `NonEmpty` value. Construct inputs with `nonempty_singleton` or
`nonempty_cons`; use `nonempty_to_list` to reason about the full ordered
input. No `DecEq` or ordering on the candidate type is required. A singleton
returns its sole value; a later candidate replaces the head only if its cost
is strictly lower.

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
expression is refined by a match. The generic strict/non-strict order bridges
are public proofs attached to `lt_nat` in `Data.Numeric.Nat.Order`, not
additional optimization laws or duplicated local recursion.

## Design notes

`argmin_from` folds the ordered tail against the head. `cheaper` tests only
the candidates' `Nat` costs, and `choose` retains its `no` value on a false
comparison. Using strict order rather than a non-strict replacement is what
makes earliest-tie behavior observable. Membership is propositionally
truncated: it certifies that the chosen value was present but is not a list
index or a Boolean equality on arbitrary candidate values. The selector
promises no complexity bound or result for an empty list.

## References

- **Arg min — Wikipedia** — https://en.wikipedia.org/wiki/Arg_max —
  orientation on the minimum-argument operation and ties among minimizers.
- **Ken optimization contract** — `spec/50-stdlib/72-optimization.md` §1 —
  the seven public laws and their proof boundary.
- **Ken natural order** — `Data.Numeric.Nat.Order` — the canonical strict
  comparison and its checked non-strict bridges.

## Trust & derivation

**Public API:** `argmin` and the seven `argmin_*` theorems. Selection helpers,
branch proofs, and the list-membership induction are private. The carrier,
its list view, the membership predicate, and Nat order retain their existing
provider identities.

| Reader task | Section |
|---|---|
| Inspect the selector and public laws | [Definition](#definition) |
| Construct a valid input | [Using it](#using-it) |
| Review membership and optimality | [Laws & proofs](#laws--proofs) |
| Locate trust and dependencies | [Trust & derivation](#trust--derivation) |

**Derivation:** `NonEmpty` and its checked list-view equality are provided by
`Data.Collections.NonEmpty`; `list_elem` and its introduction lemmas by
`Data.Collections.Derived`; `IsTrue` and `leq_nat` by
`Core.Classes.LawfulClasses`; and `lt_nat` with its two attached bridges by
`Data.Numeric.Nat.Order`. The selector adds no axiom, primitive, postulate, or
opaque constant. `trusted_base()` must stay unchanged relative to its provider
closure; this does not erase inherited assumptions.

**Validation:** `conformance/stdlib/optimization/seed-argmin.md` names
independent singleton, strict-minimum and earliest-tie observations, law
instances and false-result refusals. The scoped acceptance target checks those
client programs, generic law types and the exact trusted-base identity set.
