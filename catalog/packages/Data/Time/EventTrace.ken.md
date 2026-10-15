# Finite event traces

Finite events with natural-number times support half-open windows and a
left-biased two-list merge with checked ordering and occurrence-count laws.

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
existing trace; merging compares current heads and preserves each input's
relative order. Neither operation reads a clock or sorts unordered input.

## 2. Definition

The public carrier, operations and three generic laws lead the checked source.
The private recursive merge helper and proof lemmas follow them. Windowing
includes the lower bound but excludes the upper; a merge tie favors the left
input without examining payloads.

```ken
import Core.Classes.LawfulClasses (IsTrue, bool_and, bool_or, leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived
  (filter,
    count,
    starts_after,
    sorted_cons,
    sorted_tail,
    sorted_head,
    filter_sorted,
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

pub theorem trace_window_ordered
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr → time_ordered p (trace_window p lo hi tr) =
  filter_sorted (Event p) (event_before p) (event_before_trans p) (event_in_window p lo hi) tr

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
                    (sorted_tail (Event p) (event_before p) x xt sx)
                    sz)
              (λzs.
                λhz.
                  trace_merge_starts_after
                    p
                    x
                    xt
                    zs
                    (sorted_head (Event p) (event_before p) x xt sx)
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
          (sorted_cons
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
          (sorted_cons
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
              (sorted_head (Event p) (event_before p) y yt sorted_ys)))
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
              (sorted_tail (Event p) (event_before p) y yt sorted_ys))
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
The merge-order proof considers which input head precedes the other; totality
of natural-number order supplies the reverse comparison when the left
comparison is false. Head-order evidence is carried through recursive merge.

Occurrence count needs no ordering premise. Its induction splits on the two
head-comparison outcomes, using generic `count_cons` accounting and checked
addition associativity and commutativity. The classifier is arbitrary: it may
ignore its query and need not decide payload equality.

## 5. Design notes

`time_ordered` is a proposition over a plain list, not a field of `Event` or
of a separate trace wrapper. Unordered inputs remain legal; only the
order-preservation laws ask for ordered inputs. The timestamp order is
non-decreasing, admitting tied events. `trace_window` is half-open; equal or
inverted bounds select no events. The merge's tie rule is part of its
computation, not a consequence of the count law. No clock, duration, signed
time, infinite stream, or causality property is provided here.

## 6. References

- **Wikipedia** — [Merge algorithm](https://en.wikipedia.org/wiki/Merge_algorithm)
  — orientation on merging two already ordered inputs; this package also
  defines a result for unordered inputs by comparing current heads.
- **Ken specification** —
  [Finite event traces](../../../../spec/50-stdlib/69-time-and-events.md)
  — the carrier, half-open boundary, left-bias and three checked law contracts.
- **Collections provider** —
  [Derived collections](../../Data/Collections/Derived.ken.md)
  — generic filtering, sorted-list bridges and occurrence counting.

## 7. Trust & derivation

1. **Public API.** `Event`, `MkEvent`, its two projections, the time
   comparison, ordering predicate, window predicate, `trace_window`,
   `trace_merge`, and the three laws form the package surface.
2. **Source map.** [Definition](#2-definition) contains every public
   declaration and private support; [Using it](#3-using-it) shows an
   observable tie; [Laws & proofs](#4-laws--proofs) traces the proof families.
3. **Derivation.** `Nat` and `List` are existing carriers; windowing uses the
   derived `filter`; merging is structurally recursive on two lists; count
   preservation uses derived `count` and checked addition laws. Generic list
   lemmas are owned by the collections provider, not copied into this package.
4. **Trusted-base delta.** Zero: all three exported laws have checked proof
   bodies, and this package adds no `Axiom` or primitive. Imported logical,
   numeric and collection providers retain their inherited trust.
5. **Validation.** The
   [seed](../../../../conformance/stdlib/time-and-events/seed-event-trace.md)
   supplies independently stated closed values and one false claimed result;
   these clients complement, not replace, the generic proofs.
