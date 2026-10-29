# Exact rational intervals

Closed intervals with exact non-negative rational endpoints provide total
membership, endpoint arithmetic, hull, intersection and endpoint-wise inclusion
with checked containment and isotonicity.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Rational intervals bound uncertain non-negative values without approximating
them as floating-point numbers. Each endpoint retains its rational spelling;
membership compares values through rational order. The carrier has no proof
field imposing a sorted pair, so an inverted interval represents an empty set
of members while remaining a valid input to total endpoint arithmetic, hull
and intersection. `interval_inside` compares inner endpoints to outer endpoints
without requiring either interval to contain a member.

## 2. Definition

`Interval` holds two rational values and exports `MkInterval` so clients can
construct boxes directly. The six arithmetic and point laws lead the checked
source; their operations follow. Enclosure uses both membership premises and
the checked Rational monotonicity laws. Hull and intersection select stored
endpoint values using Rational's min/max operations; their four further laws
establish containment without assuming either input has ordered endpoints.
The endpoint-inclusion operation and its nine checked laws follow the §1–2
proofs; neither the operation nor its laws assumes sorted endpoints. Four
further checked laws establish distributivity and factoring by endpoint-value
inclusion on both sides of multiplication. Five more checked laws establish
transitivity of inclusion and operand-swap inclusion for addition,
multiplication, hull and intersection. Four further checked laws establish
both regroupings of addition and multiplication by mutual inclusion. Two
more checked laws give the least outer bound of a hull and the greatest
inner bound of an intersection, including inverted intervals.

```ken
import Core.Logic.Conjunction (And, and_intro)

import Core.Classes.LawfulClasses (IsTrue, bool_and)

import Core.Logic.Transport (sym, trans)

import Data.Numeric.Rational
  (Rational,
    MkRational,
    rational_add,
    rational_mul,
    rational_leq,
    rational_equiv,
    rational_add_mono,
    rational_mul_mono,
    rational_min,
    rational_max)

pub data Interval : Type where {
  MkInterval : (lo : Rational) → (hi : Rational) → Interval
}

export MkInterval

pub theorem interval_add_encloses
      (i : Interval) (j : Interval) (x : Rational) (y : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains j y)
      → IsTrue (interval_contains (interval_add i j) (rational_add x y)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λhx.
            λhy.
              (proof intro for bool_and)
                (rational_leq (rational_add a c) (rational_add x y))
                (rational_leq (rational_add x y) (rational_add b d))
                (rational_add_mono
                  a
                  x
                  c
                  y
                  ((proof left for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof left for bool_and) (rational_leq c y) (rational_leq y d) hy))
                (rational_add_mono
                  x
                  b
                  y
                  d
                  ((proof right for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof right for bool_and) (rational_leq c y) (rational_leq y d) hy))
      }
  }

pub theorem interval_mul_encloses
      (i : Interval) (j : Interval) (x : Rational) (y : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains j y)
      → IsTrue (interval_contains (interval_mul i j) (rational_mul x y)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λhx.
            λhy.
              (proof intro for bool_and)
                (rational_leq (rational_mul a c) (rational_mul x y))
                (rational_leq (rational_mul x y) (rational_mul b d))
                (rational_mul_mono
                  a
                  x
                  c
                  y
                  ((proof left for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof left for bool_and) (rational_leq c y) (rational_leq y d) hy))
                (rational_mul_mono
                  x
                  b
                  y
                  d
                  ((proof right for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof right for bool_and) (rational_leq c y) (rational_leq y d) hy))
      }
  }

pub theorem interval_point_contains
      (x : Rational)
    : IsTrue (interval_contains (interval_point x) x) =
  (proof intro for bool_and)
    (rational_leq x x)
    (rational_leq x x)
    ((proof refl for rational_leq) x)
    ((proof refl for rational_leq) x)

pub theorem interval_point_sound
      (x : Rational) (y : Rational)
    : IsTrue (interval_contains (interval_point x) y) → rational_equiv x y =
  λh.
    (proof antisym for rational_leq)
      x
      y
      ((proof left for bool_and) (rational_leq x y) (rational_leq y x) h)
      ((proof right for bool_and) (rational_leq x y) (rational_leq y x) h)

pub theorem interval_add_point
      (x : Rational) (y : Rational)
    : Equal Interval
        (interval_add (interval_point x) (interval_point y))
        (interval_point (rational_add x y)) =
  Refl

pub theorem interval_mul_point
      (x : Rational) (y : Rational)
    : Equal Interval
        (interval_mul (interval_point x) (interval_point y))
        (interval_point (rational_mul x y)) =
  Refl

pub fn interval_contains (i : Interval) (x : Rational) : Bool =
  match i {
    MkInterval lo hi ↦ bool_and (rational_leq lo x) (rational_leq x hi)
  }

pub fn interval_add (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_add a c) (rational_add b d)
      }
  }

pub fn interval_mul (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_mul a c) (rational_mul b d)
      }
  }

pub fn interval_point (x : Rational) : Interval = MkInterval x x

pub theorem interval_hull_contains_left
      (i : Interval) (j : Interval) (x : Rational)
    : IsTrue (interval_contains i x) → IsTrue (interval_contains (interval_hull i j) x) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λh.
            (proof intro for bool_and)
              (rational_leq (rational_min a c) x)
              (rational_leq x (rational_max b d))
              ((proof trans for rational_leq)
                (rational_min a c)
                a
                x
                ((proof leq_left for rational_min) a c)
                ((proof left for bool_and) (rational_leq a x) (rational_leq x b) h))
              ((proof trans for rational_leq)
                x
                b
                (rational_max b d)
                ((proof right for bool_and) (rational_leq a x) (rational_leq x b) h)
                ((proof left_leq for rational_max) b d))
      }
  }

pub theorem interval_hull_contains_right
      (i : Interval) (j : Interval) (x : Rational)
    : IsTrue (interval_contains j x) → IsTrue (interval_contains (interval_hull i j) x) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λh.
            (proof intro for bool_and)
              (rational_leq (rational_min a c) x)
              (rational_leq x (rational_max b d))
              ((proof trans for rational_leq)
                (rational_min a c)
                c
                x
                ((proof leq_right for rational_min) a c)
                ((proof left for bool_and) (rational_leq c x) (rational_leq x d) h))
              ((proof trans for rational_leq)
                x
                d
                (rational_max b d)
                ((proof right for bool_and) (rational_leq c x) (rational_leq x d) h)
                ((proof right_leq for rational_max) b d))
      }
  }

pub theorem interval_intersect_sound
      (i : Interval) (j : Interval) (x : Rational)
    : IsTrue (interval_contains (interval_intersect i j) x)
      → And (IsTrue (interval_contains i x)) (IsTrue (interval_contains j x)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λh.
            let
              lower_comparison = rational_leq (rational_max a c) x;
              upper_comparison = rational_leq x (rational_min b d);
              lower_witness : IsTrue lower_comparison =
                (proof left for bool_and) lower_comparison upper_comparison h;
              upper_witness : IsTrue upper_comparison =
                (proof right for bool_and) lower_comparison upper_comparison h
            in
              and_intro
                (IsTrue (bool_and (rational_leq a x) (rational_leq x b)))
                (IsTrue (bool_and (rational_leq c x) (rational_leq x d)))
                ((proof intro for bool_and)
                  (rational_leq a x)
                  (rational_leq x b)
                  ((proof trans for rational_leq)
                    a
                    (rational_max a c)
                    x
                    ((proof left_leq for rational_max) a c)
                    lower_witness)
                  ((proof trans for rational_leq)
                    x
                    (rational_min b d)
                    b
                    upper_witness
                    ((proof leq_left for rational_min) b d)))
                ((proof intro for bool_and)
                  (rational_leq c x)
                  (rational_leq x d)
                  ((proof trans for rational_leq)
                    c
                    (rational_max a c)
                    x
                    ((proof right_leq for rational_max) a c)
                    lower_witness)
                  ((proof trans for rational_leq)
                    x
                    (rational_min b d)
                    d
                    upper_witness
                    ((proof leq_right for rational_min) b d)))
      }
  }

pub theorem interval_intersect_complete
      (i : Interval) (j : Interval) (x : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains j x)
      → IsTrue (interval_contains (interval_intersect i j) x) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λhi.
            λhj.
              (proof intro for bool_and)
                (rational_leq (rational_max a c) x)
                (rational_leq x (rational_min b d))
                ((proof least for rational_max)
                  a
                  c
                  x
                  ((proof left for bool_and) (rational_leq a x) (rational_leq x b) hi)
                  ((proof left for bool_and) (rational_leq c x) (rational_leq x d) hj))
                ((proof greatest for rational_min)
                  b
                  d
                  x
                  ((proof right for bool_and) (rational_leq a x) (rational_leq x b) hi)
                  ((proof right for bool_and) (rational_leq c x) (rational_leq x d) hj))
      }
  }

pub fn interval_hull (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_min a c) (rational_max b d)
      }
  }

pub fn interval_intersect (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_max a c) (rational_min b d)
      }
  }
```

```ken
pub fn interval_inside (i : Interval) (k : Interval) : Bool =
  match i {
    MkInterval a b ↦
      match k {
        MkInterval c d ↦ bool_and (rational_leq c a) (rational_leq b d)
      }
  }

pub theorem interval_inside_sound
      (i : Interval) (k : Interval) (x : Rational)
    : IsTrue (interval_inside i k)
      → IsTrue (interval_contains i x)
      → IsTrue (interval_contains k x) =
  match i {
    MkInterval a b ↦
      match k {
        MkInterval c d ↦
          λw.
            λh.
              let
                lower_inclusion : IsTrue (rational_leq c a) =
                  (proof left for bool_and) (rational_leq c a) (rational_leq b d) w;
                upper_inclusion : IsTrue (rational_leq b d) =
                  (proof right for bool_and) (rational_leq c a) (rational_leq b d) w;
                lower_member : IsTrue (rational_leq a x) =
                  (proof left for bool_and) (rational_leq a x) (rational_leq x b) h;
                upper_member : IsTrue (rational_leq x b) =
                  (proof right for bool_and) (rational_leq a x) (rational_leq x b) h
              in
                (proof intro for bool_and)
                  (rational_leq c x)
                  (rational_leq x d)
                  ((proof trans for rational_leq) c a x lower_inclusion lower_member)
                  ((proof trans for rational_leq) x b d upper_member upper_inclusion)
      }
  }

pub theorem interval_inside_refl (i : Interval) : IsTrue (interval_inside i i) =
  match i {
    MkInterval a b ↦
      (proof intro for bool_and)
        (rational_leq a a)
        (rational_leq b b)
        ((proof refl for rational_leq) a)
        ((proof refl for rational_leq) b)
  }

pub theorem interval_inside_trans
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside i j)
      → IsTrue (interval_inside j k)
      → IsTrue (interval_inside i k) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              λij.
                λjk.
                  (proof intro for bool_and)
                    (rational_leq e a)
                    (rational_leq b f)
                    ((proof trans for rational_leq)
                      e
                      c
                      a
                      ((proof left for bool_and) (rational_leq e c) (rational_leq d f) jk)
                      ((proof left for bool_and) (rational_leq c a) (rational_leq b d) ij))
                    ((proof trans for rational_leq)
                      b
                      d
                      f
                      ((proof right for bool_and) (rational_leq c a) (rational_leq b d) ij)
                      ((proof right for bool_and) (rational_leq e c) (rational_leq d f) jk))
          }
      }
  }

pub theorem interval_add_inside
      (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval)
    : IsTrue (interval_inside i i2)
      → IsTrue (interval_inside j j2)
      → IsTrue (interval_inside (interval_add i j) (interval_add i2 j2)) =
  match i {
    MkInterval a b ↦
      match i2 {
        MkInterval a2 b2 ↦
          match j {
            MkInterval c d ↦
              match j2 {
                MkInterval c2 d2 ↦
                  λhi.
                    λhj.
                      (proof intro for bool_and)
                        (rational_leq (rational_add a2 c2) (rational_add a c))
                        (rational_leq (rational_add b d) (rational_add b2 d2))
                        (rational_add_mono
                          a2
                          a
                          c2
                          c
                          ((proof left for bool_and) (rational_leq a2 a) (rational_leq b b2) hi)
                          ((proof left for bool_and)
                            (rational_leq c2 c)
                            (rational_leq d d2)
                            hj))
                        (rational_add_mono
                          b
                          b2
                          d
                          d2
                          ((proof right for bool_and)
                            (rational_leq a2 a)
                            (rational_leq b b2)
                            hi)
                          ((proof right for bool_and)
                            (rational_leq c2 c)
                            (rational_leq d d2)
                            hj))
              }
          }
      }
  }

pub theorem interval_mul_inside
      (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval)
    : IsTrue (interval_inside i i2)
      → IsTrue (interval_inside j j2)
      → IsTrue (interval_inside (interval_mul i j) (interval_mul i2 j2)) =
  match i {
    MkInterval a b ↦
      match i2 {
        MkInterval a2 b2 ↦
          match j {
            MkInterval c d ↦
              match j2 {
                MkInterval c2 d2 ↦
                  λhi.
                    λhj.
                      (proof intro for bool_and)
                        (rational_leq (rational_mul a2 c2) (rational_mul a c))
                        (rational_leq (rational_mul b d) (rational_mul b2 d2))
                        (rational_mul_mono
                          a2
                          a
                          c2
                          c
                          ((proof left for bool_and) (rational_leq a2 a) (rational_leq b b2) hi)
                          ((proof left for bool_and)
                            (rational_leq c2 c)
                            (rational_leq d d2)
                            hj))
                        (rational_mul_mono
                          b
                          b2
                          d
                          d2
                          ((proof right for bool_and)
                            (rational_leq a2 a)
                            (rational_leq b b2)
                            hi)
                          ((proof right for bool_and)
                            (rational_leq c2 c)
                            (rational_leq d d2)
                            hj))
              }
          }
      }
  }

pub theorem interval_inside_hull_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside i (interval_hull i j)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_min a c) a)
            (rational_leq b (rational_max b d))
            ((proof leq_left for rational_min) a c)
            ((proof left_leq for rational_max) b d)
      }
  }

pub theorem interval_inside_hull_right
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside j (interval_hull i j)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_min a c) c)
            (rational_leq d (rational_max b d))
            ((proof leq_right for rational_min) a c)
            ((proof right_leq for rational_max) b d)
      }
  }

pub theorem interval_intersect_inside_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_intersect i j) i) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq a (rational_max a c))
            (rational_leq (rational_min b d) b)
            ((proof left_leq for rational_max) a c)
            ((proof leq_left for rational_min) b d)
      }
  }

pub theorem interval_intersect_inside_right
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_intersect i j) j) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq c (rational_max a c))
            (rational_leq (rational_min b d) d)
            ((proof right_leq for rational_max) a c)
            ((proof leq_right for rational_min) b d)
      }
  }

pub theorem interval_inside_complete
      (i : Interval) (k : Interval) (q : Rational)
    : IsTrue (interval_contains i q)
      → ((x : Rational) → IsTrue (interval_contains i x) → IsTrue (interval_contains k x))
      → IsTrue (interval_inside i k) =
  match i {
    MkInterval a b ↦
      match k {
        MkInterval c d ↦
          λhq.
            λall.
              let
                a_le_q : IsTrue (rational_leq a q) =
                  (proof left for bool_and) (rational_leq a q) (rational_leq q b) hq;
                q_le_b : IsTrue (rational_leq q b) =
                  (proof right for bool_and) (rational_leq a q) (rational_leq q b) hq;
                a_le_b : IsTrue (rational_leq a b) =
                  (proof trans for rational_leq) a q b a_le_q q_le_b;
                low_in_k : IsTrue (bool_and (rational_leq c a) (rational_leq a d)) =
                  all
                    a
                    ((proof intro for bool_and)
                      (rational_leq a a)
                      (rational_leq a b)
                      ((proof refl for rational_leq) a)
                      a_le_b);
                high_in_k : IsTrue (bool_and (rational_leq c b) (rational_leq b d)) =
                  all
                    b
                    ((proof intro for bool_and)
                      (rational_leq a b)
                      (rational_leq b b)
                      a_le_b
                      ((proof refl for rational_leq) b))
              in
                (proof intro for bool_and)
                  (rational_leq c a)
                  (rational_leq b d)
                  ((proof left for bool_and) (rational_leq c a) (rational_leq a d) low_in_k)
                  ((proof right for bool_and) (rational_leq c b) (rational_leq b d) high_in_k)
      }
  }

pub theorem interval_distrib_l_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_mul i (interval_add j k))
          (interval_add (interval_mul i j) (interval_mul i k))) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_add (rational_mul a c) (rational_mul a e))
                  (rational_mul a (rational_add c e)))
                (rational_leq
                  (rational_mul b (rational_add d f))
                  (rational_add (rational_mul b d) (rational_mul b f)))
                (equiv_leq
                  (rational_add (rational_mul a c) (rational_mul a e))
                  (rational_mul a (rational_add c e))
                  ((proof sym for rational_equiv)
                    (rational_mul a (rational_add c e))
                    (rational_add (rational_mul a c) (rational_mul a e))
                    ((proof distrib_l for rational_mul) a c e)))
                (equiv_leq
                  (rational_mul b (rational_add d f))
                  (rational_add (rational_mul b d) (rational_mul b f))
                  ((proof distrib_l for rational_mul) b d f))
          }
      }
  }

pub theorem interval_factor_l_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_add (interval_mul i j) (interval_mul i k))
          (interval_mul i (interval_add j k))) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_mul a (rational_add c e))
                  (rational_add (rational_mul a c) (rational_mul a e)))
                (rational_leq
                  (rational_add (rational_mul b d) (rational_mul b f))
                  (rational_mul b (rational_add d f)))
                (equiv_leq
                  (rational_mul a (rational_add c e))
                  (rational_add (rational_mul a c) (rational_mul a e))
                  ((proof distrib_l for rational_mul) a c e))
                (equiv_leq
                  (rational_add (rational_mul b d) (rational_mul b f))
                  (rational_mul b (rational_add d f))
                  ((proof sym for rational_equiv)
                    (rational_mul b (rational_add d f))
                    (rational_add (rational_mul b d) (rational_mul b f))
                    ((proof distrib_l for rational_mul) b d f)))
          }
      }
  }

pub theorem interval_distrib_r_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_mul (interval_add j k) i)
          (interval_add (interval_mul j i) (interval_mul k i))) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_add (rational_mul c a) (rational_mul e a))
                  (rational_mul (rational_add c e) a))
                (rational_leq
                  (rational_mul (rational_add d f) b)
                  (rational_add (rational_mul d b) (rational_mul f b)))
                (equiv_leq
                  (rational_add (rational_mul c a) (rational_mul e a))
                  (rational_mul (rational_add c e) a)
                  ((proof sym for rational_equiv)
                    (rational_mul (rational_add c e) a)
                    (rational_add (rational_mul c a) (rational_mul e a))
                    (rational_distrib_r a c e)))
                (equiv_leq
                  (rational_mul (rational_add d f) b)
                  (rational_add (rational_mul d b) (rational_mul f b))
                  (rational_distrib_r b d f))
          }
      }
  }

pub theorem interval_factor_r_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_add (interval_mul j i) (interval_mul k i))
          (interval_mul (interval_add j k) i)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_mul (rational_add c e) a)
                  (rational_add (rational_mul c a) (rational_mul e a)))
                (rational_leq
                  (rational_add (rational_mul d b) (rational_mul f b))
                  (rational_mul (rational_add d f) b))
                (equiv_leq
                  (rational_mul (rational_add c e) a)
                  (rational_add (rational_mul c a) (rational_mul e a))
                  (rational_distrib_r a c e))
                (equiv_leq
                  (rational_add (rational_mul d b) (rational_mul f b))
                  (rational_mul (rational_add d f) b)
                  ((proof sym for rational_equiv)
                    (rational_mul (rational_add d f) b)
                    (rational_add (rational_mul d b) (rational_mul f b))
                    (rational_distrib_r b d f)))
          }
      }
  }

pub theorem interval_add_comm_inside
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_add i j) (interval_add j i)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_add c a) (rational_add a c))
            (rational_leq (rational_add b d) (rational_add d b))
            (equiv_leq
              (rational_add c a)
              (rational_add a c)
              ((proof comm for rational_add) c a))
            (equiv_leq
              (rational_add b d)
              (rational_add d b)
              ((proof comm for rational_add) b d))
      }
  }

pub theorem interval_mul_comm_inside
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_mul i j) (interval_mul j i)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_mul c a) (rational_mul a c))
            (rational_leq (rational_mul b d) (rational_mul d b))
            (equiv_leq
              (rational_mul c a)
              (rational_mul a c)
              ((proof comm for rational_mul) c a))
            (equiv_leq
              (rational_mul b d)
              (rational_mul d b)
              ((proof comm for rational_mul) b d))
      }
  }

pub theorem interval_hull_comm_inside
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_hull i j) (interval_hull j i)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_min c a) (rational_min a c))
            (rational_leq (rational_max b d) (rational_max d b))
            ((proof greatest for rational_min)
              a
              c
              (rational_min c a)
              ((proof leq_right for rational_min) c a)
              ((proof leq_left for rational_min) c a))
            ((proof least for rational_max)
              b
              d
              (rational_max d b)
              ((proof right_leq for rational_max) d b)
              ((proof left_leq for rational_max) d b))
      }
  }

pub theorem interval_intersect_comm_inside
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_intersect i j) (interval_intersect j i)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          (proof intro for bool_and)
            (rational_leq (rational_max c a) (rational_max a c))
            (rational_leq (rational_min b d) (rational_min d b))
            ((proof least for rational_max)
              c
              a
              (rational_max a c)
              ((proof right_leq for rational_max) a c)
              ((proof left_leq for rational_max) a c))
            ((proof greatest for rational_min)
              d
              b
              (rational_min b d)
              ((proof leq_right for rational_min) b d)
              ((proof leq_left for rational_min) b d))
      }
  }

pub theorem interval_add_assoc_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_add i (interval_add j k))
          (interval_add (interval_add i j) k)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_add (rational_add a c) e)
                  (rational_add a (rational_add c e)))
                (rational_leq
                  (rational_add b (rational_add d f))
                  (rational_add (rational_add b d) f))
                (equiv_leq
                  (rational_add (rational_add a c) e)
                  (rational_add a (rational_add c e))
                  ((proof sym for rational_equiv)
                    (rational_add a (rational_add c e))
                    (rational_add (rational_add a c) e)
                    ((proof assoc for rational_add) a c e)))
                (equiv_leq
                  (rational_add b (rational_add d f))
                  (rational_add (rational_add b d) f)
                  ((proof assoc for rational_add) b d f))
          }
      }
  }

pub theorem interval_add_assoc_rev_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_add (interval_add i j) k)
          (interval_add i (interval_add j k))) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_add a (rational_add c e))
                  (rational_add (rational_add a c) e))
                (rational_leq
                  (rational_add (rational_add b d) f)
                  (rational_add b (rational_add d f)))
                (equiv_leq
                  (rational_add a (rational_add c e))
                  (rational_add (rational_add a c) e)
                  ((proof assoc for rational_add) a c e))
                (equiv_leq
                  (rational_add (rational_add b d) f)
                  (rational_add b (rational_add d f))
                  ((proof sym for rational_equiv)
                    (rational_add b (rational_add d f))
                    (rational_add (rational_add b d) f)
                    ((proof assoc for rational_add) b d f)))
          }
      }
  }

pub theorem interval_mul_assoc_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_mul i (interval_mul j k))
          (interval_mul (interval_mul i j) k)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_mul (rational_mul a c) e)
                  (rational_mul a (rational_mul c e)))
                (rational_leq
                  (rational_mul b (rational_mul d f))
                  (rational_mul (rational_mul b d) f))
                (equiv_leq
                  (rational_mul (rational_mul a c) e)
                  (rational_mul a (rational_mul c e))
                  ((proof sym for rational_equiv)
                    (rational_mul a (rational_mul c e))
                    (rational_mul (rational_mul a c) e)
                    ((proof assoc for rational_mul) a c e)))
                (equiv_leq
                  (rational_mul b (rational_mul d f))
                  (rational_mul (rational_mul b d) f)
                  ((proof assoc for rational_mul) b d f))
          }
      }
  }

pub theorem interval_mul_assoc_rev_inside
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue
        (interval_inside
          (interval_mul (interval_mul i j) k)
          (interval_mul i (interval_mul j k))) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              (proof intro for bool_and)
                (rational_leq
                  (rational_mul a (rational_mul c e))
                  (rational_mul (rational_mul a c) e))
                (rational_leq
                  (rational_mul (rational_mul b d) f)
                  (rational_mul b (rational_mul d f)))
                (equiv_leq
                  (rational_mul a (rational_mul c e))
                  (rational_mul (rational_mul a c) e)
                  ((proof assoc for rational_mul) a c e))
                (equiv_leq
                  (rational_mul (rational_mul b d) f)
                  (rational_mul b (rational_mul d f))
                  ((proof sym for rational_equiv)
                    (rational_mul b (rational_mul d f))
                    (rational_mul (rational_mul b d) f)
                    ((proof assoc for rational_mul) b d f)))
          }
      }
  }

pub theorem interval_hull_least
      (i : Interval) (j : Interval) (k : Interval)
    : IsTrue (interval_inside i k)
      → IsTrue (interval_inside j k)
      → IsTrue (interval_inside (interval_hull i j) k) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          match k {
            MkInterval e f ↦
              λik.
                λjk.
                  (proof intro for bool_and)
                    (rational_leq e (rational_min a c))
                    (rational_leq (rational_max b d) f)
                    ((proof greatest for rational_min)
                      a
                      c
                      e
                      ((proof left for bool_and) (rational_leq e a) (rational_leq b f) ik)
                      ((proof left for bool_and) (rational_leq e c) (rational_leq d f) jk))
                    ((proof least for rational_max)
                      b
                      d
                      f
                      ((proof right for bool_and) (rational_leq e a) (rational_leq b f) ik)
                      ((proof right for bool_and) (rational_leq e c) (rational_leq d f) jk))
          }
      }
  }

pub theorem interval_intersect_greatest
      (k : Interval) (i : Interval) (j : Interval)
    : IsTrue (interval_inside k i)
      → IsTrue (interval_inside k j)
      → IsTrue (interval_inside k (interval_intersect i j)) =
  match k {
    MkInterval e f ↦
      match i {
        MkInterval a b ↦
          match j {
            MkInterval c d ↦
              λki.
                λkj.
                  (proof intro for bool_and)
                    (rational_leq (rational_max a c) e)
                    (rational_leq f (rational_min b d))
                    ((proof least for rational_max)
                      a
                      c
                      e
                      ((proof left for bool_and) (rational_leq a e) (rational_leq f b) ki)
                      ((proof left for bool_and) (rational_leq c e) (rational_leq f d) kj))
                    ((proof greatest for rational_min)
                      b
                      d
                      f
                      ((proof right for bool_and) (rational_leq a e) (rational_leq f b) ki)
                      ((proof right for bool_and) (rational_leq c e) (rational_leq f d) kj))
          }
      }
  }

pub theorem interval_inside_intersect_hull_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside i (interval_intersect i (interval_hull i j))) =
  interval_intersect_greatest
    i
    i
    (interval_hull i j)
    (interval_inside_refl i)
    (interval_inside_hull_left i j)

pub theorem interval_hull_intersect_inside_left
      (i : Interval) (j : Interval)
    : IsTrue (interval_inside (interval_hull i (interval_intersect i j)) i) =
  interval_hull_least
    i
    (interval_intersect i j)
    i
    (interval_inside_refl i)
    (interval_intersect_inside_left i j)

theorem equiv_leq
      (x : Rational) (y : Rational)
    : rational_equiv x y → IsTrue (rational_leq x y) =
  λh.
    trans
      Bool
      (rational_leq x y)
      (rational_leq x x)
      True
      (sym
        Bool
        (rational_leq x x)
        (rational_leq x y)
        ((proof respect for rational_leq) x x x y ((proof refl for rational_equiv) x) h))
      ((proof refl for rational_leq) x)

theorem rational_distrib_r
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv
        (rational_mul (rational_add y z) x)
        (rational_add (rational_mul y x) (rational_mul z x)) =
  (proof trans for rational_equiv)
    (rational_mul (rational_add y z) x)
    (rational_add (rational_mul x y) (rational_mul x z))
    (rational_add (rational_mul y x) (rational_mul z x))
    ((proof trans for rational_equiv)
      (rational_mul (rational_add y z) x)
      (rational_mul x (rational_add y z))
      (rational_add (rational_mul x y) (rational_mul x z))
      ((proof comm for rational_mul) (rational_add y z) x)
      ((proof distrib_l for rational_mul) x y z))
    ((proof respect for rational_add)
      (rational_mul x y)
      (rational_mul y x)
      (rational_mul x z)
      (rational_mul z x)
      ((proof comm for rational_mul) x y)
      ((proof comm for rational_mul) x z))
```

The inclusion test compares rational values, not constructor spellings:
`interval_inside [a,b] [c,d]` holds precisely when `c ≤ a` and `b ≤ d`.
The nine laws lift membership and arithmetic through that test.

## 3. Using it

Membership includes each endpoint of a nonempty closed interval. For a point
interval, the same rational value supplies both bounds:

```ken example
const interval_unit : Rational = MkRational (Suc Zero) Zero

const interval_at_unit : Interval = MkInterval interval_unit interval_unit

theorem interval_unit_member
    : Equal Bool (interval_contains interval_at_unit interval_unit) True =
  Proved

theorem interval_unit_inside_itself
    : Equal Bool (interval_inside interval_at_unit interval_at_unit) True =
  Proved
```

`interval_add` combines lower with lower and upper with upper;
`interval_mul` does the same for multiplication. The latter formula uses
non-negative rational values and does not apply unchanged to signed values.
An inverted input still has a well-defined arithmetic result: it need not
produce another empty interval because the enclosure laws require members
of both inputs. An inverted interval may pass the endpoint-inclusion test for
a wider interval, but vacuous inclusion of empty member sets does not prove
endpoint inclusion without a member witness.

## 4. Laws & proofs

The addition and multiplication enclosure proofs split both stored endpoints,
then project the lower and upper comparisons from each membership premise.
The matching monotonicity theorem lifts the two lower comparisons to the
result's lower endpoint and the two upper comparisons to its upper endpoint.
The Boolean conjunction introduction proof assembles result membership.

Hull's two laws use the corresponding Rational min/max bounds and order
transitivity to lift a member of either input into the hull. Intersection
soundness projects membership into both inputs from the lower maximum and
upper minimum, assembling ambient Ω-valued `And`. Completeness applies the
Rational max least-upper and min greatest-lower proofs to a common member.
All four proofs use value order, not structural equality or an ordered-endpoint
premise.

Inclusion soundness composes each endpoint comparison with a member's lower
or upper comparison. Reflexivity uses rational order reflexivity on both
stored endpoints. Addition and multiplication isotonicity apply the checked
Rational monotonicity laws separately to the lower and upper bounds. Hull and
intersection inclusions use Rational min/max bounds, even for inverted inputs.
In the witnessed converse, a member `q` establishes `a ≤ q ≤ b`; this makes
both inner endpoints members, and applying pointwise inclusion to them yields
the outer bounds.

For distributivity, each law splits the three intervals and proves both
endpoint comparisons separately. The left pair uses Rational's checked
left-distributivity value law in opposite directions. The right pair uses
the private `rational_distrib_r` derivation from Rational's multiplication
commutativity and left distributivity; `equiv_leq` turns each Rational value
equivalence into the corresponding checked order comparison. Neither stage
exports a new Rational law or assumes an interval commutativity law.

Inclusion transitivity composes the two checked endpoint comparisons through
Rational order transitivity. The addition and multiplication swap laws turn
Rational value commutativity into endpoint comparisons using the existing
private `equiv_leq` proof. Hull and intersection swaps use the checked Rational
min/max bounds. Applying a swap law in each argument order gives mutual
endpoint-value inclusion, not structural equality of stored intervals.

The associativity laws split three intervals, then use Rational's checked
addition or multiplication associativity in opposite directions for the low
and high endpoints. The private `equiv_leq` bridges value equivalence into
the corresponding order comparisons. Each operation has a law in both
grouping directions, including on inverted intervals; no new Rational
export or structural interval equality is assumed.

The two bound laws split the three intervals and use Rational's checked
min-greatest and max-least proofs on opposite endpoints. A common outer
interval contains both hull operands and so contains their hull; a common
inner interval lies inside both intersection operands and so lies inside
their intersection. These endpoint-value proofs require no nonempty or
ordered-endpoint premise, including for inverted intervals.

The two absorption laws apply the bound laws without splitting endpoints.
`interval_inside_intersect_hull_left` passes `interval_intersect_greatest`
reflexivity and the left hull inclusion; `interval_hull_intersect_inside_left`
passes `interval_hull_least` reflexivity and the left intersection inclusion.
Each partner direction is an instance of one of those two inclusion laws, so
both absorptions hold as mutual endpoint-value inclusion, not structural
equality.

A point interval contains its defining value by rational order reflexivity.
Conversely, membership in a point interval forces order in both directions,
so rational antisymmetry proves value equivalence. Endpoint-wise arithmetic
of two point intervals constructs exactly the same interval as the point at
the arithmetic result; those final equations use structural `Equal Interval`.

## 5. Design notes

Bounds use `rational_leq`, not structural equality of `Rational`: different
constructor spellings can denote the same exact value. By contrast,
`Equal Interval` concerns stored endpoint spellings, so equal membership sets
need not imply structural equality. The result of arithmetic on an empty
interval is defined by its endpoint formula; no result-emptiness law is
claimed. Intersection of disjoint operands returns an inverted interval;
hull may retain a bound from an empty operand. Neither operation normalizes
equivalent rational spellings, so structural commutativity and idempotence
are not claimed. The associativity laws give mutual inclusion by endpoint
value, not a generic structural `Equal Interval` claim. Closed regroupings
can coincide in stored spelling even though open regroupings are not
convertible. Inclusion does not equate empty intervals or normalize rational
spellings; its converse requires an actual inner member.
Both distributive directions hold for inverted inputs without asserting
structural `Equal Interval`: different endpoint spellings may denote equal
Rational values. Subtraction, signed arithmetic, division near zero,
floating rounding and width remain outside this non-negative exact interval
API.

## 6. References

- **Wikipedia** —
  [Interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic)
  — orientation on interval enclosures; this exact non-negative formulation
  deliberately has a smaller operation set.
- **Ken specification** —
  [Exact numerical intervals](../../../../spec/50-stdlib/68-numerical.md)
  — normative contract for inclusive membership, endpoint arithmetic and laws.
- **Rational provider** — [Exact rational values](../../Data/Numeric/Rational.ken.md)
  — the value order, monotonicity and min/max proofs reused here.

## 7. Trust & derivation

1. **Public API.** `Interval`, `MkInterval`, `interval_contains`,
   `interval_add`, `interval_mul`, `interval_point`, `interval_hull`,
   `interval_intersect`, `interval_inside` and thirty-six checked interval
   laws form the public contract.
2. **Source map.** [Definition](#2-definition) has every exported law and
   operation, [Using it](#3-using-it) checks construction and membership,
   and [Laws & proofs](#4-laws--proofs) explains the proof families,
   including endpoint inclusion, its witnessed converse and the four
   distributivity/factoring inclusions.
3. **Derivation.** Carrier endpoints and exact arithmetic come from the
   rational package. Enclosure uses its public checked two-input addition
   and multiplication monotonicity; point soundness uses rational order
   reflexivity and antisymmetry. Hull and intersection reuse the six public
   Rational min/max bounds, with no local copy of scalar order proofs.
   Inclusion soundness reuses transitivity; its isotonicity reuses Rational
   monotonicity, and its four bound laws reuse those min/max proofs. The
   four distributive inclusions reuse Rational's checked `distrib_l` and
   `comm` laws through two private proof stages. Inclusion transitivity reuses
   Rational order transitivity; the four swap inclusions reuse Rational
   commutativity or public min/max bounds without a new helper.
4. **Trusted-base delta.** Zero: all thirty-six laws have checked proof bodies,
   and the package declares no `Axiom` or new primitive. Imported rational,
   natural-number and logical providers retain their inherited trust.
5. **Validation.** The
   [arithmetic seed](../../../../conformance/stdlib/numerical/seed-rational-interval.md)
   [hull/intersection seed](../../../../conformance/stdlib/numerical/seed-interval-hull-intersection.md)
   [inclusion seed](../../../../conformance/stdlib/numerical/seed-interval-inclusion.md)
   [distributivity seed](../../../../conformance/stdlib/numerical/seed-interval-distributivity.md)
   [inclusion-order seed](../../../../conformance/stdlib/numerical/seed-interval-inside-order.md)
   [associativity seed](../../../../conformance/stdlib/numerical/seed-interval-associativity-inside.md)
   [bound-laws seed](../../../../conformance/stdlib/numerical/seed-interval-hull-intersect-bounds.md)
   and [absorption seed](../../../../conformance/stdlib/numerical/seed-interval-hull-intersect-absorption.md)
   supply separately constructed positive values and false controls;
   those finite cases do not replace the generic checked proof bodies. The
   distributivity seed's accepting client and dropped-product refusal, and the
   inclusion-order seed's accepted client and two chain refusals, run in package
   tests. The associativity seed's whole accepted client, including its two
   closed `Refl` controls, and isolated false prefix-inclusion claim are checked
   by the candidate's `cat_numeric_interval_associativity_inside` package test.
   The §7 package test root-loads both generic bound proofs, checks the whole
   accepted bound-laws seed and refuses its isolated false hull claim.
   The §8 `cat_numeric_interval_hull_intersect_absorption` package test checks
   the complete accepted absorption seed and its isolated moved-argument
   refusal against the two new public laws.
   These package tests do not establish official conformance-runner execution,
   CI, publication or landing.
