# `Collections` — derived collection, string, and byte views

The derived `List`/`Nat` combinator floor (`spec/30-surface/37-strings-collections.md
§2.4/§2.5/§2.5.1/§4.1), the CAT-3 structural/verified-sort/projection-abstraction
slices, the 5 derived `String` ops built on top of the real
`string_to_list_char`/`list_char_to_string` round trip. Every combinator here
is a termination-checked recursive derived definition over the real generic
eliminator — zero new kernel feature, zero `trusted_base()` delta anywhere in
this file. The `Bytes` structural view also supplies `bytes_nat_length`, an
ordinary fold through `bytes_to_list` with no cached length or local `Axiom`.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws  proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [Findings](#6-findings)
7. [References](#7-references)
8. [Trust  derivation](#8-trust--derivation)

## 1. Motivation

`spec/30-surface/37-strings-collections.md §2.4/§2.5/§2.5.1/§4.1` (WP
`L3-strings-surface`, slice 2/2 of the string surface — slice 1,
`L3-strings-roundtrip`, landed the real `string_to_list_char`/
`list_char_to_string` round trip this package rides) needs a `List`/`Nat`
combinator floor and five derived `String` operations, stated once as
ordinary checked Ken rather than re-derived per consumer. The floor has seven
combinators. Saturating `Nat` subtraction is the canonical
`sub` operation imported from `Data.Numeric.Nat.Order` and described in `§4.5`
below.
Every combinator, law, and string op is a termination-checked recursive
`declare_def` (upgrading opaque to transparent on `sct_check` success) over
the real generic eliminator — a `match` on `List`/`Nat` lowers to a real
`Term::Elim{fam}` (`34 §3`); there is no `elim_List`/`elim_Nat` constant, and
no native interpreter primitive is added for any of these combinators
(Approach A, Architect ruling `evt_4k1yqah3yvpds`: a native primitive would
grow the tested-not-trusted reduction surface for a trivially structural
fold, a subsume-don't-proliferate violation).

## 2. Definition

`OrdResult` is the canonical 3-way comparison result (`§2.5.1`), imported
from `Core.Logic.OrdResult`. Structural `Pair` and `List` comparison lives in
`Core.Logic.Compare`, below both collection packages and lawful class
instances, so every consumer reuses the same provider identities. The landed
`Ord Char` (`catalog/packages/Core/Classes/LawfulClasses.ken.md`) is
`leq`-only — no `compare` method — so `compare_char` below packages its existing
`eqChar` and `leqChar` operations into the canonical result type.

The first four of the seven floor combinators follow: `list_append`
(deliberately a distinct name from the landed `Bytes`-domain `append`,
FS-effect, `crates/ken-elaborator/src/bytes.rs` — this is the pure
`List a -> List a -> List a` op and must not shadow or be shadowed by it),
`nth`, `take`, and `drop`. `list_eq` and `list_compare` are imported from
the canonical comparison provider; `sub` is imported from the canonical Nat
order provider and described in `§4.5`, next to the string ops that need it.

```ken
import Data.Numeric.Nat.Order (min, sub)

import Core.Logic.Compare (list_compare, list_eq)

import Core.Classes.LawfulClasses (IsTrue, bool_and, bool_or, bool_leq, leq_nat)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.OrdResult
  (OrdResult,
    Lt,
    Eq,
    Gt,
    ord_eq,
    ord_lt,
    ord_gt,
    ord_result_leq,
    ord_result_dispatch2,
    ord_result_elim,
    ord_result_elim2)

import Core.Logic.Transport (cong, sym, trans)

pub fn list_append (a : Type) (xs : List a) (ys : List a) : List a =
  match xs {
    Nil ↦ ys;
    Cons x xs2 ↦ Cons a x (list_append a xs2 ys)
  }

pub fn nth (a : Type) (n : Nat) (xs : List a) : Option a =
  match xs {
    Nil ↦ None a;
    Cons h t ↦
      match n {
        Zero ↦ Some a h;
        Suc m ↦ nth a m t
      }
  }

fn take (a : Type) (n : Nat) (xs : List a) : List a =
  match n {
    Zero ↦ Nil a;
    Suc m ↦
      match xs {
        Nil ↦ Nil a;
        Cons h t ↦ Cons a h (take a m t)
      }
  }

fn drop (a : Type) (n : Nat) (xs : List a) : List a =
  match n {
    Zero ↦ xs;
    Suc m ↦
      match xs {
        Nil ↦ Nil a;
        Cons h t ↦ drop a m t
      }
  }
```

## 3. Using it

This package builds up in four layers, each riding the one before: the
floor above; `§4.1`'s structural ops (`map`/`filter`/`mem`/`length`/`min`);
`§4.3`'s verified `List Bool` insertion sort, a caller-facing example of
proving a concrete instantiation of the generic `sort`/`insert` sound and
permutation-preserving; and `§4.6`'s 5 derived `String` ops
(`concat`/`slice`/`char_at`/`eq`/`compare`), which every later catalog
package that manipulates `String` values reaches for directly. Every proof
term in `§4` uses `cong`/`sym`/`trans` from
`catalog/packages/Core/Logic/Transport.ken.md`, so a consumer loads Transport
before this file.

## 4. Laws  proofs

### 4.1 CAT-3 D1 — structural list operations

Ordinary transparent recursive definitions over `List`/`Nat`; no primitive
or postulated law is added. `take_drop_decomposition`, `map_length`, and
`length_take_min` are the three original proof-returning laws. The private
`mem_filter` theorem characterizes membership through the installed prelude
`filter`: given `compat`, a matching head has the same predicate result as
`x`. The private `mem_filter_sound` theorem needs no compatibility premise:
membership after filtering implies membership before filtering. Both laws
range over any element type, comparator, predicate, query and list; neither
adds a wrapper or a new trust assumption. The two attached `nth` laws connect
successful lookup and out-of-bounds lookup to the structural `length` fold in
both directions.

Migrated here per the attached-proof ownership rule — an attached proof
`f::law` belongs to the module that defines `f` — the three `list_append`
monoid laws `left_unit`/`assoc`/`right_unit` are proved beside `list_append`
as well, so every `Monoid (List a)` instance cites the one canonical
function-level proof. Left unit is definitional: `list_append (Nil a) x`
iota-reduces to `x`, so the goal stays `Eq`-shaped and closes by `Refl`.
Assoc is induction on the first list — base reduces both sides to the neutral
`list_append a ys zs` (`Refl`), step lifts the tail IH under `Cons a h` with
`cong`. Right unit is induction on the list — base reduces both sides to the
constructor `Nil a`, which observationally collapses to `Top` and closes by
`Proved`; step is `cong` under `Cons a h` on the tail IH.

```ken
fn mem (a : Type) (eqf : a → a → Bool) (x : a) (xs : List a) : Bool =
  match xs {
    Nil ↦ False;
    Cons h t ↦
      match eqf x h {
        True ↦ True;
        False ↦ mem a eqf x t
      }
  }

pub fn length (a : Type) (xs : List a) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons h t ↦ Suc (length a t)
  }

pub proof some_below_length for nth
      (a : Type) (n : Nat) (xs : List a)
    : (v : a)
      → Equal (Option a) (nth a n xs) (Some a v)
      → IsTrue (leq_nat (Suc n) (length a xs)) =
  match xs {
    Nil ↦ λv. λh. absurd h;
    Cons head tail ↦
      match n {
        Zero ↦ λv. λh. Proved;
        Suc n2 ↦ λv. λh. (proof some_below_length for nth) a n2 tail v h
      }
  }

pub proof at_or_beyond_is_none for nth
      (a : Type) (n : Nat) (xs : List a)
    : IsTrue (leq_nat (length a xs) n) → Equal (Option a) (nth a n xs) (None a) =
  match xs {
    Nil ↦ λh. Proved;
    Cons head tail ↦
      match n {
        Zero ↦ λh. absurd h;
        Suc n2 ↦ λh. (proof at_or_beyond_is_none for nth) a n2 tail h
      }
  }

theorem take_drop_decomposition
      (a : Type) (n : Nat) (xs : List a)
    : Equal (List a) (list_append a (take a n xs) (drop a n xs)) xs =
  match n {
    Zero ↦ Refl;
    Suc m ↦
      match xs {
        Nil ↦ Proved;
        Cons h t ↦
          cong
            (List a)
            (List a)
            (list_append a (take a m t) (drop a m t))
            t
            (Cons a h)
            (take_drop_decomposition a m t)
      }
  }

pub proof left_unit for list_append
      (a : Type) (xs : List a)
    : Equal (List a) (list_append a (Nil a) xs) xs =
  Refl

pub proof assoc for list_append
      (a : Type) (xs : List a) (ys : List a) (zs : List a)
    : Equal
        (List a)
        (list_append a (list_append a xs ys) zs)
        (list_append a xs (list_append a ys zs)) =
  match xs {
    Nil ↦ Refl;
    Cons h t ↦
      cong
        (List a)
        (List a)
        (list_append a (list_append a t ys) zs)
        (list_append a t (list_append a ys zs))
        (Cons a h)
        ((proof assoc for list_append) a t ys zs)
  }

pub proof right_unit for list_append
      (a : Type) (xs : List a)
    : Equal (List a) (list_append a xs (Nil a)) xs =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      cong
        (List a)
        (List a)
        (list_append a t (Nil a))
        t
        (Cons a h)
        ((proof right_unit for list_append) a t)
  }

theorem bool_and_false_right (b : Bool) : Equal Bool (bool_and b False) False =
  match b {
    True ↦ Proved;
    False ↦ Proved
  }

theorem mem_filter_head_true
      (a : Type)
      (eqf : a → a → Bool)
      (p : a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (eb : Equal Bool (p h) True)
    : Equal Bool
        (mem a eqf x (filter a p (Cons a h t)))
        (mem a eqf x (Cons a h (filter a p t))) =
  cong
    Bool
    Bool
    (p h)
    True
    (λq.
      mem
        a
        eqf
        x
        (match q {
          True ↦ Cons a h (filter a p t);
          False ↦ filter a p t
        }))
    eb

theorem mem_filter_head_false
      (a : Type)
      (eqf : a → a → Bool)
      (p : a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (eb : Equal Bool (p h) False)
    : Equal Bool (mem a eqf x (filter a p (Cons a h t))) (mem a eqf x (filter a p t)) =
  cong
    Bool
    Bool
    (p h)
    False
    (λq.
      mem
        a
        eqf
        x
        (match q {
          True ↦ Cons a h (filter a p t);
          False ↦ filter a p t
        }))
    eb

theorem mem_cons_head_true
      (a : Type)
      (eqf : a → a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (ec : Equal Bool (eqf x h) True)
    : Equal Bool (mem a eqf x (Cons a h t)) True =
  cong
    Bool
    Bool
    (eqf x h)
    True
    (λq.
      match q {
        True ↦ True;
        False ↦ mem a eqf x t
      })
    ec

theorem mem_cons_head_false
      (a : Type)
      (eqf : a → a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (ec : Equal Bool (eqf x h) False)
    : Equal Bool (mem a eqf x (Cons a h t)) (mem a eqf x t) =
  cong
    Bool
    Bool
    (eqf x h)
    False
    (λq.
      match q {
        True ↦ True;
        False ↦ mem a eqf x t
      })
    ec

theorem mem_filter_cons_case
      (a : Type)
      (eqf : a → a → Bool)
      (p : a → Bool)
      (x : a)
      (compat : (y : a) → IsTrue (eqf x y) → Equal Bool (p y) (p x))
      (h : a)
      (t : List a)
      (ih : Equal Bool (mem a eqf x (filter a p t)) (bool_and (mem a eqf x t) (p x)))
      (b : Bool)
      (c : Bool)
    : Equal Bool (p h) b
      → Equal Bool (eqf x h) c
      → Equal Bool
        (mem a eqf x (filter a p (Cons a h t)))
        (bool_and (mem a eqf x (Cons a h t)) (p x)) =
  match b {
    True ↦
      match c {
        True ↦
          λeb.
            λec.
              trans
                Bool
                (mem a eqf x (filter a p (Cons a h t)))
                (mem a eqf x (Cons a h (filter a p t)))
                (bool_and (mem a eqf x (Cons a h t)) (p x))
                (mem_filter_head_true a eqf p x h t eb)
                (trans
                  Bool
                  (mem a eqf x (Cons a h (filter a p t)))
                  True
                  (bool_and (mem a eqf x (Cons a h t)) (p x))
                  (mem_cons_head_true a eqf x h (filter a p t) ec)
                  (trans
                    Bool
                    True
                    (p x)
                    (bool_and (mem a eqf x (Cons a h t)) (p x))
                    (trans Bool True (p h) (p x) (sym Bool (p h) True eb) (compat h ec))
                    (sym
                      Bool
                      (bool_and (mem a eqf x (Cons a h t)) (p x))
                      (p x)
                      (cong
                        Bool
                        Bool
                        (mem a eqf x (Cons a h t))
                        True
                        (λq. bool_and q (p x))
                        (mem_cons_head_true a eqf x h t ec)))));
        False ↦
          λeb.
            λec.
              trans
                Bool
                (mem a eqf x (filter a p (Cons a h t)))
                (mem a eqf x (Cons a h (filter a p t)))
                (bool_and (mem a eqf x (Cons a h t)) (p x))
                (mem_filter_head_true a eqf p x h t eb)
                (trans
                  Bool
                  (mem a eqf x (Cons a h (filter a p t)))
                  (mem a eqf x (filter a p t))
                  (bool_and (mem a eqf x (Cons a h t)) (p x))
                  (mem_cons_head_false a eqf x h (filter a p t) ec)
                  (trans
                    Bool
                    (mem a eqf x (filter a p t))
                    (bool_and (mem a eqf x t) (p x))
                    (bool_and (mem a eqf x (Cons a h t)) (p x))
                    ih
                    (sym
                      Bool
                      (bool_and (mem a eqf x (Cons a h t)) (p x))
                      (bool_and (mem a eqf x t) (p x))
                      (cong
                        Bool
                        Bool
                        (mem a eqf x (Cons a h t))
                        (mem a eqf x t)
                        (λq. bool_and q (p x))
                        (mem_cons_head_false a eqf x h t ec)))))
      };
    False ↦
      match c {
        True ↦
          λeb.
            λec.
              trans
                Bool
                (mem a eqf x (filter a p (Cons a h t)))
                False
                (bool_and (mem a eqf x (Cons a h t)) (p x))
                (trans
                  Bool
                  (mem a eqf x (filter a p (Cons a h t)))
                  (bool_and (mem a eqf x t) (p x))
                  False
                  (trans
                    Bool
                    (mem a eqf x (filter a p (Cons a h t)))
                    (mem a eqf x (filter a p t))
                    (bool_and (mem a eqf x t) (p x))
                    (mem_filter_head_false a eqf p x h t eb)
                    ih)
                  (trans
                    Bool
                    (bool_and (mem a eqf x t) (p x))
                    (bool_and (mem a eqf x t) False)
                    False
                    (cong
                      Bool
                      Bool
                      (p x)
                      False
                      (λq. bool_and (mem a eqf x t) q)
                      (trans Bool (p x) (p h) False (sym Bool (p h) (p x) (compat h ec)) eb))
                    (bool_and_false_right (mem a eqf x t))))
                (sym
                  Bool
                  (bool_and (mem a eqf x (Cons a h t)) (p x))
                  False
                  (trans
                    Bool
                    (bool_and (mem a eqf x (Cons a h t)) (p x))
                    (p x)
                    False
                    (cong
                      Bool
                      Bool
                      (mem a eqf x (Cons a h t))
                      True
                      (λq. bool_and q (p x))
                      (mem_cons_head_true a eqf x h t ec))
                    (trans Bool (p x) (p h) False (sym Bool (p h) (p x) (compat h ec)) eb)));
        False ↦
          λeb.
            λec.
              trans
                Bool
                (mem a eqf x (filter a p (Cons a h t)))
                (mem a eqf x (filter a p t))
                (bool_and (mem a eqf x (Cons a h t)) (p x))
                (mem_filter_head_false a eqf p x h t eb)
                (trans
                  Bool
                  (mem a eqf x (filter a p t))
                  (bool_and (mem a eqf x t) (p x))
                  (bool_and (mem a eqf x (Cons a h t)) (p x))
                  ih
                  (sym
                    Bool
                    (bool_and (mem a eqf x (Cons a h t)) (p x))
                    (bool_and (mem a eqf x t) (p x))
                    (cong
                      Bool
                      Bool
                      (mem a eqf x (Cons a h t))
                      (mem a eqf x t)
                      (λq. bool_and q (p x))
                      (mem_cons_head_false a eqf x h t ec))))
      }
  }

theorem mem_filter
      (a : Type)
      (eqf : a → a → Bool)
      (p : a → Bool)
      (x : a)
      (compat : (y : a) → IsTrue (eqf x y) → Equal Bool (p y) (p x))
      (xs : List a)
    : Equal Bool (mem a eqf x (filter a p xs)) (bool_and (mem a eqf x xs) (p x)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      mem_filter_cons_case
        a
        eqf
        p
        x
        compat
        h
        t
        (mem_filter a eqf p x compat t)
        (p h)
        (eqf x h)
        (Refl)
        (Refl)
  }

theorem mem_filter_sound_cons_case
      (a : Type)
      (eqf : a → a → Bool)
      (p : a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (ih : IsTrue (mem a eqf x (filter a p t)) → IsTrue (mem a eqf x t))
      (b : Bool)
      (c : Bool)
    : Equal Bool (p h) b
      → Equal Bool (eqf x h) c
      → IsTrue (mem a eqf x (filter a p (Cons a h t)))
      → IsTrue (mem a eqf x (Cons a h t)) =
  match c {
    True ↦ λeb. λec. λhm. mem_cons_head_true a eqf x h t ec;
    False ↦
      match b {
        True ↦
          λeb.
            λec.
              λhm.
                trans
                  Bool
                  (mem a eqf x (Cons a h t))
                  (mem a eqf x t)
                  True
                  (mem_cons_head_false a eqf x h t ec)
                  (ih
                    (trans
                      Bool
                      (mem a eqf x (filter a p t))
                      (mem a eqf x (filter a p (Cons a h t)))
                      True
                      (sym
                        Bool
                        (mem a eqf x (filter a p (Cons a h t)))
                        (mem a eqf x (filter a p t))
                        (trans
                          Bool
                          (mem a eqf x (filter a p (Cons a h t)))
                          (mem a eqf x (Cons a h (filter a p t)))
                          (mem a eqf x (filter a p t))
                          (mem_filter_head_true a eqf p x h t eb)
                          (mem_cons_head_false a eqf x h (filter a p t) ec)))
                      hm));
        False ↦
          λeb.
            λec.
              λhm.
                trans
                  Bool
                  (mem a eqf x (Cons a h t))
                  (mem a eqf x t)
                  True
                  (mem_cons_head_false a eqf x h t ec)
                  (ih
                    (trans
                      Bool
                      (mem a eqf x (filter a p t))
                      (mem a eqf x (filter a p (Cons a h t)))
                      True
                      (sym
                        Bool
                        (mem a eqf x (filter a p (Cons a h t)))
                        (mem a eqf x (filter a p t))
                        (mem_filter_head_false a eqf p x h t eb))
                      hm))
      }
  }

theorem mem_filter_sound
      (a : Type) (eqf : a → a → Bool) (p : a → Bool) (x : a) (xs : List a)
    : IsTrue (mem a eqf x (filter a p xs)) → IsTrue (mem a eqf x xs) =
  match xs {
    Nil ↦ λhm. absurd hm;
    Cons h t ↦
      mem_filter_sound_cons_case
        a
        eqf
        p
        x
        h
        t
        (mem_filter_sound a eqf p x t)
        (p h)
        (eqf x h)
        (Refl)
        (Refl)
  }

theorem map_length
      (a : Type) (b : Type) (f : a → b) (xs : List a)
    : Equal Nat (length b (map a b f xs)) (length a xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦ cong Nat Nat (length b (map a b f t)) (length a t) Suc (map_length a b f t)
  }

theorem length_take_min
      (a : Type) (n : Nat) (xs : List a)
    : Equal Nat (length a (take a n xs)) (min n (length a xs)) =
  match n {
    Zero ↦ Proved;
    Suc m ↦
      match xs {
        Nil ↦ Proved;
        Cons h t ↦
          cong Nat Nat (length a (take a m t)) (min m (length a t)) Suc (length_take_min a m t)
      }
  }
```

### 4.2 DS-4 — five more `List` combinators completing the floor

`reverse`/`zip`/`concat_map`/`range`/`foldl`, each an ordinary
structural-recursion `fn` in the same style as `§4.1` — zero new kernel
feature, zero `trusted_base()` delta. `reverse` is naive, `list_append`-based
(not an accumulator): this spelling makes the involutive proof cleanest,
needing only the standard reverse-of-snoc helper lemma below, not a separate
accumulator-invariant lemma. Both `Nil` branches of `reverse_snoc` close via
`cong _ _ (Cons a y)` over the fully-collapsed `Nil = Nil` — a `Cons`-vs-`Cons`
goal with an ABSTRACT shared element `y` does not itself collapse to bare
`Top` (the kernel's own equality-at-inductive reduction produces a
right-nested Σ pairing the stuck, `y`-abstract element equality with the
collapsed tail equality, so `Proved`/`Refl` alone both fail); lifting `Proved`
through `Cons` via `cong` is the direct, minimal proof. The checked
`reverse::involutive` law is available alongside `reverse` to recover a
list from its reversed view; `reverse_snoc` remains an internal lemma.
`zip` truncates at the shorter list (`Nil` on either empty), NOT the
length-indexed `Vec` zip:
this is ordinary non-dependent recursion carrying none of the
sibling-convoy/dependent-match capability gate that a length-indexed zip
would need — fully mechanical. `concat_map` retains its two structural
(`Nil`/`Cons`) equations and a private distributivity proof over
`list_append`. Its step lifts the tail
induction hypothesis through head-list append, then reverses
`list_append::assoc` to meet the other side. There is no bespoke length law:
that would need a `sum` combinator not in this floor
(subsume-don't-proliferate).
`range n` produces `[0, 1, .., n-1]` via a `start`-threaded helper
(`range_from`) so the recursion is structural on `n` while the contents
count up. `foldl` similarly ships with only its two structural equations —
no `foldl`/`foldr` relationship law, since `foldr` is not in this floor and
inventing one solely to state a law here would be exactly the proliferation
this package avoids elsewhere.

```ken
pub fn reverse (a : Type) (xs : List a) : List a =
  match xs {
    Nil ↦ Nil a;
    Cons h t ↦ list_append a (reverse a t) (Cons a h (Nil a))
  }

theorem reverse_snoc
      (a : Type) (xs : List a) (y : a)
    : Equal
        (List a)
        (reverse a (list_append a xs (Cons a y (Nil a))))
        (Cons a y (reverse a xs)) =
  match xs {
    Nil ↦ cong (List a) (List a) (Nil a) (Nil a) (Cons a y) Proved;
    Cons h t ↦
      cong
        (List a)
        (List a)
        (reverse a (list_append a t (Cons a y (Nil a))))
        (Cons a y (reverse a t))
        (λw. list_append a w (Cons a h (Nil a)))
        (reverse_snoc a t y)
  }

pub proof involutive for reverse
      (a : Type) (xs : List a)
    : Equal (List a) (reverse a (reverse a xs)) xs =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      trans
        (List a)
        (reverse a (reverse a (Cons a h t)))
        (Cons a h (reverse a (reverse a t)))
        (Cons a h t)
        (reverse_snoc a (reverse a t) h)
        (cong
          (List a)
          (List a)
          (reverse a (reverse a t))
          t
          (Cons a h)
          ((proof involutive for reverse) a t))
  }

theorem append_length_snoc
      (a : Type) (xs : List a) (y : a)
    : Equal Nat (length a (list_append a xs (Cons a y (Nil a)))) (Suc (length a xs)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      cong
        Nat
        Nat
        (length a (list_append a t (Cons a y (Nil a))))
        (Suc (length a t))
        Suc
        (append_length_snoc a t y)
  }

theorem reverse_length
      (a : Type) (xs : List a)
    : Equal Nat (length a (reverse a xs)) (length a xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      trans
        Nat
        (length a (list_append a (reverse a t) (Cons a h (Nil a))))
        (Suc (length a (reverse a t)))
        (Suc (length a t))
        (append_length_snoc a (reverse a t) h)
        (cong Nat Nat (length a (reverse a t)) (length a t) Suc (reverse_length a t))
  }

fn zip (a : Type) (b : Type) (xs : List a) (ys : List b) : List (Pair a b) =
  match xs {
    Nil ↦ Nil (Pair a b);
    Cons h t ↦
      match ys {
        Nil ↦ Nil (Pair a b);
        Cons h2 t2 ↦ Cons (Pair a b) (mk_pair a b h h2) (zip a b t t2)
      }
  }

theorem zip_length
      (a : Type) (b : Type) (xs : List a) (ys : List b)
    : Equal Nat (length (Pair a b) (zip a b xs ys)) (min (length a xs) (length b ys)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match ys {
        Nil ↦ Proved;
        Cons h2 t2 ↦
          cong
            Nat
            Nat
            (length (Pair a b) (zip a b t t2))
            (min (length a t) (length b t2))
            Suc
            (zip_length a b t t2)
      }
  }

pub fn concat_map (a : Type) (b : Type) (f : a → List b) (xs : List a) : List b =
  match xs {
    Nil ↦ Nil b;
    Cons h t ↦ list_append b (f h) (concat_map a b f t)
  }

theorem concat_map_append
      (a : Type) (b : Type) (f : a → List b) (xs : List a) (ys : List a)
    : Equal
        (List b)
        (concat_map a b f (list_append a xs ys))
        (list_append b (concat_map a b f xs) (concat_map a b f ys)) =
  match xs {
    Nil ↦ Refl;
    Cons h t ↦
      let
        head_segment = f h;
        mapped_tail = concat_map a b f t;
        mapped_suffix = concat_map a b f ys;
        mapped_appended_tail = concat_map a b f (list_append a t ys);
        right_associated = list_append b head_segment (list_append b mapped_tail mapped_suffix);
        left_associated = list_append b (list_append b head_segment mapped_tail) mapped_suffix
      in
        trans
          (List b)
          (list_append b head_segment mapped_appended_tail)
          right_associated
          left_associated
          (cong
            (List b)
            (List b)
            mapped_appended_tail
            (list_append b mapped_tail mapped_suffix)
            (λw. list_append b head_segment w)
            (concat_map_append a b f t ys))
          (sym
            (List b)
            left_associated
            right_associated
            ((proof assoc for list_append) b head_segment mapped_tail mapped_suffix))
  }

fn range_from (start : Nat) (n : Nat) : List Nat =
  match n {
    Zero ↦ Nil Nat;
    Suc m ↦ Cons Nat start (range_from (Suc start) m)
  }

fn range (n : Nat) : List Nat = range_from Zero n

theorem range_from_length
      (start : Nat) (n : Nat)
    : Equal Nat (length Nat (range_from start n)) n =
  match n {
    Zero ↦ Proved;
    Suc m ↦
      cong
        Nat
        Nat
        (length Nat (range_from (Suc start) m))
        m
        Suc
        (range_from_length (Suc start) m)
  }

theorem range_length (n : Nat) : Equal Nat (length Nat (range n)) n = range_from_length Zero n

fn foldl (a : Type) (b : Type) (f : b → a → b) (z : b) (xs : List a) : b =
  match xs {
    Nil ↦ z;
    Cons h t ↦ foldl a b f (f z h) t
  }
```

### 4.3 CAT-3 D2 — generic insertion sort and `List Bool` laws

`Perm` is intentionally the package-local count/multiset equality surface —
an ordinary `Prop`-valued function over an explicit comparator, never a raw
proof-relevant inductive family — not the older prelude truncation
relation; a consumer that loads this package gets the executable
comparator-indexed form the verified `List Bool` carrier needs. `insert`/
`sort` are the ordinary transparent generic combinators; the proofs below
specialize them to `List Bool` under `bool_leq`, showing the specialized
`sort_bool` (a direct case-split implementation, not `sort` applied to
`bool_leq`) is both order-preserving (`sort_bool_sorted`) and a genuine
permutation of its input (`sort_bool_perm`, via the two count-preservation
lemmas for `True`/`False`). The generic `insert` preserves every count for
any comparator, and `sort` preserves counts and is sorted when the comparator
is total; no transitivity or comparator-indexed equality is required.

```ken
pub fn eq_from_ord (a : Type) (le : a → a → Bool) (x : a) (y : a) : Bool =
  bool_and (le x y) (le y x)

pub fn count (a : Type) (eqf : a → a → Bool) (x : a) (xs : List a) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons h t ↦
      match eqf x h {
        True ↦ Suc (count a eqf x t);
        False ↦ count a eqf x t
      }
  }

fn Perm (a : Type) (eqf : a → a → Bool) (xs : List a) (ys : List a) : Prop =
  (x : a) → Equal Nat (count a eqf x xs) (count a eqf x ys)

fn insert (a : Type) (le : a → a → Bool) (x : a) (xs : List a) : List a =
  match xs {
    Nil ↦ Cons a x (Nil a);
    Cons h t ↦
      match le x h {
        True ↦ Cons a x (Cons a h t);
        False ↦ Cons a h (insert a le x t)
      }
  }

fn sort (a : Type) (le : a → a → Bool) (xs : List a) : List a =
  match xs {
    Nil ↦ Nil a;
    Cons h t ↦ insert a le h (sort a le t)
  }

fn derived_sort_head_ordered (a : Type) (le : a → a → Bool) (x : a) (xs : List a) : Prop =
  match xs {
    Nil ↦ Top;
    Cons h t ↦ Equal Bool (le x h) True
  }

theorem derived_sort_sorted_cons
      (a : Type) (le : a → a → Bool) (x : a) (xs : List a)
    : is_sorted a le xs → derived_sort_head_ordered a le x xs → is_sorted a le (Cons a x xs) =
  match xs {
    Nil ↦ λsorted_xs. λhead_before. Proved;
    Cons h t ↦
      λsorted_xs.
        λhead_before.
          and_intro
            (Equal Bool (le x h) True)
            (is_sorted a le (Cons a h t))
            head_before
            sorted_xs
  }

theorem derived_sort_sorted_tail
      (a : Type) (le : a → a → Bool) (x : a) (xs : List a)
    : is_sorted a le (Cons a x xs) → is_sorted a le xs =
  match xs {
    Nil ↦ λsorted_input. Proved;
    Cons h t ↦
      λsorted_input.
        and_snd (Equal Bool (le x h) True) (is_sorted a le (Cons a h t)) sorted_input
  }

theorem derived_sort_sorted_head
      (a : Type) (le : a → a → Bool) (x : a) (xs : List a)
    : is_sorted a le (Cons a x xs) → derived_sort_head_ordered a le x xs =
  match xs {
    Nil ↦ λsorted_input. Proved;
    Cons h t ↦
      λsorted_input.
        and_fst (Equal Bool (le x h) True) (is_sorted a le (Cons a h t)) sorted_input
  }

theorem derived_sort_right_of_left_false
      (a : Type)
      (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (x : a)
      (y : a)
      (left_false : Equal Bool (le x y) False)
    : Equal Bool (le y x) True =
  trans
    Bool
    (le y x)
    (bool_or (le x y) (le y x))
    True
    (sym
      Bool
      (bool_or (le x y) (le y x))
      (le y x)
      (cong Bool Bool (le x y) False (λdecision. bool_or decision (le y x)) left_false))
    (total x y)

theorem derived_sort_head_after_insert_cons_case
      (a : Type) (le : a → a → Bool) (h : a) (x : a) (y : a) (ys : List a) (decision : Bool)
    : Equal Bool (le x y) decision
      → Equal Bool (le h x) True
      → derived_sort_head_ordered a le h (Cons a y ys)
      → derived_sort_head_ordered a le h (insert a le x (Cons a y ys)) =
  match decision {
    True ↦
      λcomparison.
        λh_before_x.
          λh_before_y.
            J
              (λchoice _.
                derived_sort_head_ordered
                  a
                  le
                  h
                  (match choice {
                    True ↦ Cons a x (Cons a y ys);
                    False ↦ Cons a y (insert a le x ys)
                  }))
              h_before_x
              (sym Bool (le x y) True comparison);
    False ↦
      λcomparison.
        λh_before_x.
          λh_before_y.
            J
              (λchoice _.
                derived_sort_head_ordered
                  a
                  le
                  h
                  (match choice {
                    True ↦ Cons a x (Cons a y ys);
                    False ↦ Cons a y (insert a le x ys)
                  }))
              h_before_y
              (sym Bool (le x y) False comparison)
  }

theorem derived_sort_head_after_insert
      (a : Type) (le : a → a → Bool) (h : a) (x : a) (xs : List a)
    : Equal Bool (le h x) True
      → derived_sort_head_ordered a le h xs
      → derived_sort_head_ordered a le h (insert a le x xs) =
  match xs {
    Nil ↦ λh_before_x. λh_before_xs. h_before_x;
    Cons y ys ↦
      λh_before_x.
        λh_before_y.
          derived_sort_head_after_insert_cons_case
            a
            le
            h
            x
            y
            ys
            (le x y)
            Refl
            h_before_x
            h_before_y
  }

theorem derived_sort_insert_sorted_cons_case
      (a : Type)
      (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (x : a)
      (h : a)
      (t : List a)
      (ih : is_sorted a le t → is_sorted a le (insert a le x t))
      (decision : Bool)
    : Equal Bool (le x h) decision
      → is_sorted a le (Cons a h t)
      → is_sorted a le (insert a le x (Cons a h t)) =
  match decision {
    True ↦
      λcomparison.
        λsorted_xs.
          J
            (λdecision _.
              is_sorted
                a
                le
                (match decision {
                  True ↦ Cons a x (Cons a h t);
                  False ↦ Cons a h (insert a le x t)
                }))
            (derived_sort_sorted_cons a le x (Cons a h t) sorted_xs comparison)
            (sym Bool (le x h) True comparison);
    False ↦
      λcomparison.
        λsorted_xs.
          let
            tail_is_sorted = derived_sort_sorted_tail a le h t sorted_xs;
            inserted_tail_is_sorted = ih tail_is_sorted;
            h_before_x = derived_sort_right_of_left_false a le total x h comparison;
            h_before_tail = derived_sort_sorted_head a le h t sorted_xs;
            h_before_inserted_tail =
              derived_sort_head_after_insert a le h x t h_before_x h_before_tail;
            branch_is_sorted =
              derived_sort_sorted_cons
                a
                le
                h
                (insert a le x t)
                inserted_tail_is_sorted
                h_before_inserted_tail
          in
            J
              (λdecision _.
                is_sorted
                  a
                  le
                  (match decision {
                    True ↦ Cons a x (Cons a h t);
                    False ↦ Cons a h (insert a le x t)
                  }))
              branch_is_sorted
              (sym Bool (le x h) False comparison)
  }

proof sorted for insert
      (a : Type)
      (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (x : a)
      (xs : List a)
    : is_sorted a le xs → is_sorted a le (insert a le x xs) =
  match xs {
    Nil ↦ λsorted_xs. Proved;
    Cons h t ↦
      derived_sort_insert_sorted_cons_case
        a
        le
        total
        x
        h
        t
        (insert::sorted a le total x t)
        (le x h)
        Refl
  }

proof sorted for sort
      (a : Type)
      (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (xs : List a)
    : is_sorted a le (sort a le xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦ insert::sorted a le total h (sort a le t) (sort::sorted a le total t)
  }

theorem derived_sort_count_cons_cong
      (a : Type)
      (eqf : a → a → Bool)
      (q : a)
      (h : a)
      (xs : List a)
      (ys : List a)
      (counts_equal : Equal Nat (count a eqf q xs) (count a eqf q ys))
    : Equal Nat (count a eqf q (Cons a h xs)) (count a eqf q (Cons a h ys)) =
  match eqf q h eqn : occurrence {
    True ↦
      J
        (λdecision _.
          Equal
            Nat
            (match decision {
              True ↦ Suc (count a eqf q xs);
              False ↦ count a eqf q xs
            })
            (match decision {
              True ↦ Suc (count a eqf q ys);
              False ↦ count a eqf q ys
            }))
        (cong Nat Nat (count a eqf q xs) (count a eqf q ys) Suc counts_equal)
        (sym Bool (eqf q h) True occurrence);
    False ↦
      J
        (λdecision _.
          Equal
            Nat
            (match decision {
              True ↦ Suc (count a eqf q xs);
              False ↦ count a eqf q xs
            })
            (match decision {
              True ↦ Suc (count a eqf q ys);
              False ↦ count a eqf q ys
            }))
        counts_equal
        (sym Bool (eqf q h) False occurrence)
  }

fn derived_sort_count_after_two
      (tail_count : Nat) (first_occurs : Bool) (second_occurs : Bool)
    : Nat =
  match first_occurs {
    True ↦
      Suc
        (match second_occurs {
          True ↦ Suc tail_count;
          False ↦ tail_count
        });
    False ↦
      match second_occurs {
        True ↦ Suc tail_count;
        False ↦ tail_count
      }
  }

theorem derived_sort_count_swap_decisions
      (tail_count : Nat) (x_occurs : Bool) (y_occurs : Bool)
    : Equal Nat
        (derived_sort_count_after_two tail_count x_occurs y_occurs)
        (derived_sort_count_after_two tail_count y_occurs x_occurs) =
  match x_occurs {
    True ↦
      match y_occurs {
        True ↦ Refl;
        False ↦ Refl
      };
    False ↦
      match y_occurs {
        True ↦ Refl;
        False ↦ Refl
      }
  }

theorem derived_sort_count_cons_swap
      (a : Type) (eqf : a → a → Bool) (q : a) (x : a) (y : a) (xs : List a)
    : Equal Nat
        (count a eqf q (Cons a x (Cons a y xs)))
        (count a eqf q (Cons a y (Cons a x xs))) =
  derived_sort_count_swap_decisions (count a eqf q xs) (eqf q x) (eqf q y)

theorem derived_sort_insert_count_cons_case
      (a : Type)
      (le : a → a → Bool)
      (x : a)
      (h : a)
      (t : List a)
      (eqf : a → a → Bool)
      (q : a)
      (ih : Equal Nat (count a eqf q (Cons a x t)) (count a eqf q (insert a le x t)))
      (decision : Bool)
    : Equal Bool (le x h) decision
      → Equal Nat
        (count a eqf q (Cons a x (Cons a h t)))
        (count a eqf q (insert a le x (Cons a h t))) =
  match decision {
    True ↦
      λcomparison.
        J
          (λdecision _.
            Equal
              Nat
              (count a eqf q (Cons a x (Cons a h t)))
              (count
                a
                eqf
                q
                (match decision {
                  True ↦ Cons a x (Cons a h t);
                  False ↦ Cons a h (insert a le x t)
                })))
          Refl
          (sym Bool (le x h) True comparison);
    False ↦
      λcomparison.
        let
          swapped_count = derived_sort_count_cons_swap a eqf q x h t;
          recursive_count = ih;
          inserted_count =
            derived_sort_count_cons_cong
              a
              eqf
              q
              h
              (Cons a x t)
              (insert a le x t)
              recursive_count;
          branch_count =
            trans
              Nat
              (count a eqf q (Cons a x (Cons a h t)))
              (count a eqf q (Cons a h (Cons a x t)))
              (count a eqf q (Cons a h (insert a le x t)))
              swapped_count
              inserted_count
        in
          J
            (λdecision _.
              Equal
                Nat
                (count a eqf q (Cons a x (Cons a h t)))
                (count
                  a
                  eqf
                  q
                  (match decision {
                    True ↦ Cons a x (Cons a h t);
                    False ↦ Cons a h (insert a le x t)
                  })))
            branch_count
            (sym Bool (le x h) False comparison)
  }

proof count for insert
      (a : Type) (le : a → a → Bool) (x : a) (xs : List a) (eqf : a → a → Bool) (q : a)
    : Equal Nat (count a eqf q (Cons a x xs)) (count a eqf q (insert a le x xs)) =
  match xs {
    Nil ↦ Refl;
    Cons h t ↦
      derived_sort_insert_count_cons_case
        a
        le
        x
        h
        t
        eqf
        q
        (insert::count a le x t eqf q)
        (le x h)
        Refl
  }

proof perm for sort
      (a : Type) (le : a → a → Bool) (xs : List a) (eqf : a → a → Bool)
    : Perm a eqf xs (sort a le xs) =
  match xs {
    Nil ↦ λq. Proved;
    Cons h t ↦
      λq.
        let
          original_count = count a eqf q (Cons a h t);
          tail_sorted_count = count a eqf q (Cons a h (sort a le t));
          final_count = count a eqf q (insert a le h (sort a le t));
          tail_counts_equal =
            derived_sort_count_cons_cong a eqf q h t (sort a le t) (sort::perm a le t eqf q);
          insertion_counts_equal = insert::count a le h (sort a le t) eqf q
        in
          trans
            Nat
            original_count
            tail_sorted_count
            final_count
            tail_counts_equal
            insertion_counts_equal
  }

fn bool_head_leq (x : Bool) (xs : List Bool) : Prop =
  match xs {
    Nil ↦ Top;
    Cons h t ↦ Equal Bool (bool_leq x h) True
  }

proof false for bool_head_leq (xs : List Bool) : bool_head_leq False xs =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦ Proved
  }

theorem bool_cons_sorted
      (x : Bool) (xs : List Bool)
    : is_sorted Bool bool_leq xs
      → bool_head_leq x xs
      → is_sorted Bool bool_leq (Cons Bool x xs) =
  match xs {
    Nil ↦ λh. λhb. Proved;
    Cons h t ↦
      λhxs.
        λhb.
          and_intro
            (Equal Bool (bool_leq x h) True)
            (is_sorted Bool bool_leq (Cons Bool h t))
            hb
            hxs
  }

proof tail for is_sorted
      (x : Bool) (xs : List Bool)
    : is_sorted Bool bool_leq (Cons Bool x xs) → is_sorted Bool bool_leq xs =
  match xs {
    Nil ↦ λh. Proved;
    Cons h t ↦
      λhCons.
        and_snd (Equal Bool (bool_leq x h) True) (is_sorted Bool bool_leq (Cons Bool h t)) hCons
  }

fn insert_true_bool (xs : List Bool) : List Bool =
  match xs {
    Nil ↦ Cons Bool True (Nil Bool);
    Cons h t ↦
      match h {
        True ↦ Cons Bool True (Cons Bool True t);
        False ↦ Cons Bool False (insert_true_bool t)
      }
  }

fn sort_bool (xs : List Bool) : List Bool =
  match xs {
    Nil ↦ Nil Bool;
    Cons h t ↦
      match h {
        False ↦ Cons Bool False (sort_bool t);
        True ↦ insert_true_bool (sort_bool t)
      }
  }

theorem sorted_insert_true_bool
      (xs : List Bool)
    : is_sorted Bool bool_leq xs → is_sorted Bool bool_leq (insert_true_bool xs) =
  match xs {
    Nil ↦ λh. Proved;
    Cons h t ↦
      match h {
        True ↦ λhxs. bool_cons_sorted True (Cons Bool True t) hxs Proved;
        False ↦
          λhxs.
            bool_cons_sorted
              False
              (insert_true_bool t)
              (sorted_insert_true_bool t ((proof tail for is_sorted) False t hxs))
              ((proof false for bool_head_leq) (insert_true_bool t))
      }
  }

theorem sort_bool_sorted (xs : List Bool) : is_sorted Bool bool_leq (sort_bool xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match h {
        False ↦
          let
            sorted_tail = sort_bool t;
            tail_is_sorted = sort_bool_sorted t;
            false_precedes_tail = (proof false for bool_head_leq) sorted_tail
          in
            bool_cons_sorted False sorted_tail tail_is_sorted false_precedes_tail;
        True ↦
          let
            sorted_tail = sort_bool t;
            tail_is_sorted = sort_bool_sorted t
          in
            sorted_insert_true_bool sorted_tail tail_is_sorted
      }
  }

theorem insert_true_bool_count_false
      (xs : List Bool)
    : Equal Nat
        (count Bool (eq_from_ord Bool bool_leq) False (insert_true_bool xs))
        (count Bool (eq_from_ord Bool bool_leq) False xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match h {
        True ↦ Refl;
        False ↦
          cong
            Nat
            Nat
            (count Bool (eq_from_ord Bool bool_leq) False (insert_true_bool t))
            (count Bool (eq_from_ord Bool bool_leq) False t)
            Suc
            (insert_true_bool_count_false t)
      }
  }

theorem insert_true_bool_count_true
      (xs : List Bool)
    : Equal Nat
        (count Bool (eq_from_ord Bool bool_leq) True (insert_true_bool xs))
        (Suc (count Bool (eq_from_ord Bool bool_leq) True xs)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match h {
        True ↦ Refl;
        False ↦ insert_true_bool_count_true t
      }
  }

theorem sort_bool_count_false
      (xs : List Bool)
    : Equal Nat
        (count Bool (eq_from_ord Bool bool_leq) False xs)
        (count Bool (eq_from_ord Bool bool_leq) False (sort_bool xs)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match h {
        False ↦
          cong
            Nat
            Nat
            (count Bool (eq_from_ord Bool bool_leq) False t)
            (count Bool (eq_from_ord Bool bool_leq) False (sort_bool t))
            Suc
            (sort_bool_count_false t);
        True ↦
          trans
            Nat
            (count Bool (eq_from_ord Bool bool_leq) False t)
            (count Bool (eq_from_ord Bool bool_leq) False (sort_bool t))
            (count Bool (eq_from_ord Bool bool_leq) False (insert_true_bool (sort_bool t)))
            (sort_bool_count_false t)
            (sym
              Nat
              (count Bool (eq_from_ord Bool bool_leq) False (insert_true_bool (sort_bool t)))
              (count Bool (eq_from_ord Bool bool_leq) False (sort_bool t))
              (insert_true_bool_count_false (sort_bool t)))
      }
  }

theorem sort_bool_count_true
      (xs : List Bool)
    : Equal Nat
        (count Bool (eq_from_ord Bool bool_leq) True xs)
        (count Bool (eq_from_ord Bool bool_leq) True (sort_bool xs)) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      match h {
        False ↦ sort_bool_count_true t;
        True ↦
          trans
            Nat
            (Suc (count Bool (eq_from_ord Bool bool_leq) True t))
            (Suc (count Bool (eq_from_ord Bool bool_leq) True (sort_bool t)))
            (count Bool (eq_from_ord Bool bool_leq) True (insert_true_bool (sort_bool t)))
            (cong
              Nat
              Nat
              (count Bool (eq_from_ord Bool bool_leq) True t)
              (count Bool (eq_from_ord Bool bool_leq) True (sort_bool t))
              Suc
              (sort_bool_count_true t))
            (sym
              Nat
              (count Bool (eq_from_ord Bool bool_leq) True (insert_true_bool (sort_bool t)))
              (Suc (count Bool (eq_from_ord Bool bool_leq) True (sort_bool t)))
              (insert_true_bool_count_true (sort_bool t)))
      }
  }

theorem sort_bool_perm
      (xs : List Bool)
    : Perm Bool (eq_from_ord Bool bool_leq) xs (sort_bool xs) =
  match xs {
    Nil ↦
      λq.
        match q {
          False ↦ Proved;
          True ↦ Proved
        };
    Cons h t ↦
      λq.
        match q {
          False ↦ sort_bool_count_false (Cons Bool h t);
          True ↦ sort_bool_count_true (Cons Bool h t)
        }
  }
```

### 4.4 CAT-3 D3 — projection-abstraction classes

Ordinary class records over the landed right-nested Σ record machinery.
The concrete lens is intentionally over `Pair Bool Bool` only — a
polymorphic `Lens s a` / `Iso a b` and quotient-carrier views need surface
support that is not part of this slice. Every law field below closes by
`Refl`, since each concrete operation (`fst_pair_bool_bool`,
`set_fst_pair_bool_bool`, the two `Bool`-identity functions) reduces
definitionally once applied.

```ken
class View A {
  project : A → A
}

class Lens A {
  get : Pair Bool Bool → Bool;
  set : Bool → Pair Bool Bool → Pair Bool Bool;
  get_set : (a : Bool) → (s : Pair Bool Bool) → Equal Bool (get (set a s)) a;
  set_get : (s : Pair Bool Bool) → Equal (Pair Bool Bool) (set (get s) s) s;
  set_set :
    (a : Bool)
    → (b : Bool)
    → (s : Pair Bool Bool)
    → Equal (Pair Bool Bool) (set b (set a s)) (set b s)
}

class Iso A {
  to : Bool → Bool;
  from : Bool → Bool;
  to_from : (x : Bool) → Equal Bool (to (from x)) x;
  from_to : (x : Bool) → Equal Bool (from (to x)) x
}

class Representation A {
  encode : Bool → Bool;
  decode : Bool → Bool;
  roundtrip : (x : Bool) → Equal Bool (decode (encode x)) x
}

class RefinementView A {
  project : ({b : Bool | Equal Bool b True}) → Bool
}

class IndexedView A {
  project : Pair Bool Bool → Bool → Bool
}

class SetoidMorphism A {
  project : Bool → Bool;
  respects : (x : Bool) → (y : Bool) → (Equal Bool x y) → Equal Bool (project x) (project y)
}

fn id_bool (x : Bool) : Bool = x

fn fst_pair_bool_bool (p : Pair Bool Bool) : Bool = pair_fst Bool Bool p

fn set_fst_pair_bool_bool (a : Bool) (p : Pair Bool Bool) : Pair Bool Bool =
  mk_pair Bool Bool a (pair_snd Bool Bool p)

theorem fst_lens_get_set
      (a : Bool) (s : Pair Bool Bool)
    : Equal Bool (fst_pair_bool_bool (set_fst_pair_bool_bool a s)) a =
  Refl

theorem fst_lens_set_get
      (s : Pair Bool Bool)
    : Equal (Pair Bool Bool) (set_fst_pair_bool_bool (fst_pair_bool_bool s) s) s =
  Refl

proof set_set for set_fst_pair_bool_bool
      (a : Bool) (b : Bool) (s : Pair Bool Bool)
    : Equal
        (Pair Bool Bool)
        (set_fst_pair_bool_bool b (set_fst_pair_bool_bool a s))
        (set_fst_pair_bool_bool b s) =
  Refl

instance View Bool {
  project = id_bool
}

instance Lens Unit {
  get = fst_pair_bool_bool;
  set = set_fst_pair_bool_bool;
  get_set = fst_lens_get_set;
  set_get = fst_lens_set_get;
  set_set = proof set_set for set_fst_pair_bool_bool
}

fn bool_iso_to (x : Bool) : Bool = x

fn bool_iso_from (x : Bool) : Bool = x

theorem bool_iso_to_from (x : Bool) : Equal Bool (bool_iso_to (bool_iso_from x)) x = Refl

theorem bool_iso_from_to (x : Bool) : Equal Bool (bool_iso_from (bool_iso_to x)) x = Refl

instance Iso Unit {
  to = bool_iso_to;
  from = bool_iso_from;
  to_from = bool_iso_to_from;
  from_to = bool_iso_from_to
}

instance Representation Unit {
  encode = bool_iso_to;
  decode = bool_iso_from;
  roundtrip = bool_iso_from_to
}

fn true_refinement_project (x : {b : Bool | Equal Bool b True}) : Bool = x

instance RefinementView Unit {
  project = true_refinement_project
}

fn bool_pair_index_project (p : Pair Bool Bool) (ix : Bool) : Bool =
  match ix {
    False ↦ pair_fst Bool Bool p;
    True ↦ pair_snd Bool Bool p
  }

instance IndexedView Unit {
  project = bool_pair_index_project
}

proof respects for id_bool
      (x : Bool) (y : Bool)
    : Equal Bool x y → Equal Bool (id_bool x) (id_bool y) =
  λp. p

instance SetoidMorphism Unit {
  project = id_bool;
  respects = proof respects for id_bool
}
```

### 4.5 The remaining floor combinators

`sub`, imported from the canonical `Data.Numeric.Nat.Order` provider, is
saturating `Nat` monus and never underflows. `§4.6`'s `slice` needs exactly
this structural computation for its length. The imported `list_eq` and
`list_compare` complete the seven-combinator floor from `§1`.
`compare_char` is a faithful 3-way repackaging of the landed `leqChar`/`eqChar`
(`crates/ken-elaborator/src/decimal_char.rs`, Rust-side primitives — not
catalog declarations, so their own names are untouched by this catalog's
casing convention), not a re-derivation of `Char` comparison (settled input
#4, `docs/program/wp/L3-strings-surface.md §2`): `eqChar` decides equality
directly; otherwise `Lt`/`Gt` follow from `leqChar`'s antisymmetry and
totality (both landed `Ord Char` laws, by transport from `Ord Int`).

```ken
fn compare_char (a : Char) (b : Char) : OrdResult =
  match eqChar a b {
    True ↦ ord_eq;
    False ↦
      match leqChar a b {
        True ↦ ord_lt;
        False ↦ ord_gt
      }
  }
```

### 4.6 The 5 derived `String` ops

Routed through the real `string_to_list_char`/`list_char_to_string` round
trip (slice 1). These ship as plain functions — `eq`/`compare` are
tested-not-trusted Boolean/decision ops, not lawful `DecEq String`/
`Ord String` instances (that transport needs a lawful `DecEq Char`, not yet
landed — a tracked follow-on; filing these as proof-carrying instances now
would over-claim the trust level). `slice` clamps by construction: `drop`
past the end yields `Nil`, `take` past the end stops at the end, and the
length `sub j i` saturates at `0` when `j < i` — never an underflow,
never stuck. `char_at` is total and honest about absence — `Option Char`,
never a sentinel or a partial index. `eq` is codepoint-wise equality over
the scalar sequence, riding the landed `eqChar` — this is never
NFC-normalization equality (ADR 0010 §3: that identifies distinct scalar
sequences, so over the codepoint carrier it is non-canonical — a lawful
`DecEq` for it would inhabit `Bottom`). `compare` is 3-way, codepoint-wise
lexicographic order — the more fundamental op, subsuming `<=`/`<`/`==`
(a `leq`-only interface cannot cheaply recover a 3-way result).

```ken
fn concat (a : String) (b : String) : String =
  list_char_to_string (list_append Char (string_to_list_char a) (string_to_list_char b))

fn slice (i : Nat) (j : Nat) (s : String) : String =
  let
    characters = string_to_list_char s;
    suffix = drop Char i characters;
    slice_width = sub j i;
    selected_window = take Char slice_width suffix
  in
    list_char_to_string selected_window

fn char_at (i : Nat) (s : String) : Option Char = nth Char i (string_to_list_char s)

fn eq (a : String) (b : String) : Bool =
  list_eq Char eqChar (string_to_list_char a) (string_to_list_char b)

fn compare (a : String) (b : String) : OrdResult =
  list_compare Char compare_char (string_to_list_char a) (string_to_list_char b)
```

### 4.7 The derived `Bytes` structural fold

`bytes_nat_length` is ordinary checked Ken over the `List UInt8` view. It
reuses the already-termination-checked generic `length` fold above; the only
trusted cost is the fixed `Bytes ↔ List UInt8` primitive pair and its two
registered round-trip propositions. This definition adds no primitive,
postulate, cached-`Nat` carrier, or `Axiom`.

```ken
pub fn bytes_nat_length (bs : Bytes) : Nat = length UInt8 (bytes_to_list bs)
```

## 5. Design notes

**Package dependency.** The CAT-3 proof terms in `§4.1`–`§4.3` use `cong`/
`sym`/`trans`, so harnesses and consumers load
`catalog/packages/Core/Logic/Transport.ken.md` before this file. The dependency is
proof-only and adds no trusted-base delta.

**SCT sound zone.** Every recursive call in this package is an applied call
whose decreasing argument is a strict subterm of a matched argument (the
`Cons` tail and/or the `Suc` predecessor) — squarely in the termination
checker's sound zone, never leaning on its unapplied-self-reference /
recursion-through-opaque-map over-accept hole.

**Deliverability honesty.** `String` is canonical with respect to `List Char`:
the landed String-side retraction makes `string_to_list_char` injective
(ADR 0010 §2). Therefore `DecEq String`/`Ord String` instances are soundly
deliverable later — but that transport additionally needs a lawful `DecEq
Char`, which is now landed in `Core/Classes/LawfulClasses`. Filing
`eq`/`compare` as proof-carrying instances here would still over-claim the
trust level; this package ships the functions only, honestly.

## 6. Findings

- **Kernel-reduction defect:** none.
- **Abstraction candidate:** `§4.1` now proves private `mem_filter` with an
  explicit comparator/predicate compatibility premise, and proves private
  `mem_filter_sound` without that premise. Both address the installed prelude
  `filter`; neither publishes a new wrapper.
- **Runtime-performance characteristic (non-blocking, forward-tracked).**
  `crates/ken-elaborator/tests/l3_strings_surface_acceptance.rs`'s
  `derived_string_ops_reduce_over_real_roundtrip` test exercises the pinned
  `slice 0 99 "abc"` equivalent-to-`"abc"` conformance case
  (`conformance/surface/collections/seed-collections.md` DS-AC3). Evaluating
  a `take`/`drop`-style structural recursion at a unary-`Nat` depth of ~99
  costs noticeably more than linear time in the current `ken-interp`
  evaluator (empirically ~O(n^3.5–4) in the recursion depth `n`, not
  exponential — a correct value, just slow: this one test takes on the
  order of a few CPU-minutes at `n = 99`, versus sub-millisecond at
  `n <= 40`). This is a pre-existing characteristic of `ken-interp`'s
  reduction strategy for deep unary-`Nat` recursion under nested `match` (no
  prior test exercised `Nat` depths anywhere near this range), **not** a bug
  introduced by this package's derived definitions (the combinators are
  correct and match the spec's mandated shapes exactly), and **not** a
  soundness concern (the interpreter is the tested-not-trusted ring — a
  wrong value, never a false proof, and the value here is correct). Flagged
  to the language-leader/Architect as a forward-tracked `ken-interp`
  performance finding; not a blocker for this package.

## 7. References

None — this entry's design is Ken-native, not consulted from an external
reference implementation.

## 8. Trust  derivation

1. **Spec / WP.** `spec/30-surface/37-strings-collections.md §2.4/§2.5/
   §2.5.1/§4.1`; WP `L3-strings-surface` (this package, slice 2/2);
   `L3-strings-roundtrip` (slice 1, the native round trip this rides).
2. **Public API.** `OrdResult`, `list_append`, `nth`, `take`, `drop`,
   `sub`, `list_eq`, `list_compare` (the 7-combinator floor); `map`,
   `filter`, `mem`, `length`, `min`, `take_drop_decomposition`,
   `map_length`, `length_take_min` (CAT-3 D1); `nth::some_below_length`,
   `nth::at_or_beyond_is_none` (the two lookup bounds); `reverse`,
   `reverse::involutive`,
   `zip`, `concat_map`, `range`, `foldl` and their proofs (DS-4); `count`,
   `Perm`, `insert`, `sort`, `sort_bool`, `sort_bool_sorted`,
   `sort_bool_perm` (CAT-3 D2); `View`, `Lens`, `Iso`, `Representation`,
   `RefinementView`, `IndexedView`, `SetoidMorphism` (CAT-3 D3);
   `compare_char`, `concat`, `slice`, `char_at`, `eq`, `compare` (the 5
   derived `String` ops).
3. **Source map.**

   | Task | Section |
   |---|---|
   | See the floor's first four combinators | [Definition](#2-definition) |
   | See how the layers build on each other | [Using it](#3-using-it) |
   | Structural laws, verified sort, projection classes, the string ops | [Laws  proofs](#4-laws--proofs) |
   | Package dependency, SCT sound zone, deliverability honesty | [Design notes](#5-design-notes) |

4. **Derivation path.** Every combinator, law, and string op is a
   `declare_def` (checked, upgraded opaque to transparent on `sct_check`
   success) or an ordinary `fn`; `OrdResult` is a checked `data` inductive
   (kernel-admitted by positivity), never a primitive or postulated
   declaration. No native interpreter primitive is added for any list
   combinator/law or string op (Approach A, Architect ruling
   `evt_4k1yqah3yvpds`) — deriving trivially structural folds keeps the
   audited primitive set small (subsume-don't-proliferate).
5. **`trusted_base()` delta.** **Zero beyond imported providers.** Every
   proof in this package is a genuine, kernel-checked term; no law field is
   postulated here. The roots-loaded provider closure already contains five
   opaque assumptions: `Ord Int`'s `refl`, `antisym`, `trans`, and `total`, and
   `StringBijection`'s `string_to_list_char_retraction`. The private
   `concat_map_append` proof uses structural induction and the checked
   `list_append::assoc`, `cong`, `sym`, and `trans` proofs, not these axioms.
   The generic sort proofs consume an explicit totality premise only for
   sortedness; they neither add an axiom nor invoke the inherited `Ord Int`
   assumptions.
6. **Proof families.** `§4.1`/`§4.2`: structural induction + `cong`/`trans`
   lifting the tail IH under the head constructor; private `mem_filter`
   and `mem_filter_sound` split named predicate/comparator outcomes and use
   `cong` over the prelude `filter` branch, with compatibility needed only
   for the first law. Private `concat_map_append` lifts the IH under
   `list_append` and uses
   `list_append::assoc` in reverse. The `nth` bounds proofs split the list
   before the index so lookup, length, and order reduce together. `§4.3`:
   generic `insert::count` preserves every count with any comparator and
   `sort::perm` composes the tail and insertion equalities. The generic
   `insert::sorted` and `sort::sorted` use only comparator totality; private
   decision-variable helpers rewrite applied comparator branches without
   relying on a proof-side match to refine the goal. The separate `List Bool`
   proofs case-split under `bool_leq`, closing by `Proved`/`Refl`/`cong`/
   `trans`/`sym`. `§4.4`: every law field closes by
   `Refl` (each concrete operation reduces definitionally once applied, no
   case-split needed).
7. **Consumers.** `catalog/packages/Data/Collections/Map.ken` (the proved
   `Map`/`Set` BST) depends on this package's `list_append`.
   `crates/ken-elaborator/tests/cat1_lawful_functors_package.rs`,
   `ds3_sum_combinators_acceptance.rs`, `ds4_list_combinators_acceptance.rs`,
   `ds7_applicative_monad_acceptance.rs`, `ds8_traversable_acceptance.rs`,
   `either_catalog_package_acceptance.rs`, `es2_acceptance.rs`,
   `l3_strings_surface_acceptance.rs`, `map_build_acceptance.rs`, and
   `cat3_collections_package.rs` all load this package as a cross-file
   prerequisite for their own consuming packages; `crates/ken-cli/tests/rosetta.rs`
   concatenates it (after `Transport.ken.md`'s tangled source) ahead of
   several rosetta examples that reuse it per the DRY rule.
8. **Validation evidence.**
   `crates/ken-elaborator/tests/cat_derived_filter_membership_law.rs` —
   pins both private checked contracts to the installed prelude `filter`
   and distinguishes incompatible from compatible concrete equations.
   `crates/ken-elaborator/tests/cat_derived_sort_laws.rs` — pins four
   attached private raw contracts to Derived's own `insert`/`sort`, checks
   generic consumers and a total versus non-total comparator on one fixture.
   `crates/ken-elaborator/tests/cat3_collections_package.rs` — confirms the
   CAT-3 D1/D2/D3 surface elaborates with zero `trusted_base()` delta, that
   every law is proof-returning (not a bare `Prop` wrapper) and postulates
   nothing, and that `§4.4`'s classes stay capitalized (no stray lowercase
   `View`-style declaration reintroduced).
   `crates/ken-elaborator/tests/ds4_list_combinators_acceptance.rs` —
   confirms the DS-4 combinators register as real globals and the file
   postulates nothing. `crates/ken-elaborator/tests/l3_strings_surface_acceptance.rs` —
   confirms the 5 derived `String` ops reduce over the real round trip.
