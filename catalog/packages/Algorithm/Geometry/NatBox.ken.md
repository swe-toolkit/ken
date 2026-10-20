# Closed boxes over natural coordinates

Points and axis-aligned boxes over `Nat` have total, exact membership and
geometry operations. Six box-and-distance laws, two metric laws, and
seven corner-wise containment laws carry kernel-checked generic proofs.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A closed box stores two corners. Each coordinate is a natural number, so
membership is two inclusive comparisons per axis. Inverting either pair of
bounds represents an empty box without making operations partial or equating
all empty boxes structurally. The corners remain observable even when the
point set is empty.

## 2. Definition

The two constructors are exported separately from their carrier types so a
client can build a point and a box. Five original operations and six
box-and-distance laws lead the definition; two metric laws follow. The
corner-wise `inside` operation and its seven laws come next. Their
per-axis proof helpers and distance helpers stay private.

Intersection takes coordinatewise maximum lower and minimum upper bounds;
hull takes the opposite bounds. Distance adds two saturated absolute
coordinate differences. `inside b c` compares both corners of `b` with
those of `c`, even if `b` is inverted and has no points. Every proof body
below is the checked general construction, not a finite seed in place of
a proof.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat, bool_and)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (min, max, sub, leq_nat_add_left_bound, leq_nat_weaken_right)

import Core.Logic.Transport (cong, sym, trans)

pub data Point = MkPoint Nat Nat

export MkPoint

pub data Box = MkBox Point Point

export MkBox

pub fn axis_contains (lo : Nat) (hi : Nat) (x : Nat) : Bool =
  bool_and (leq_nat lo x) (leq_nat x hi)

pub fn contains (b : Box) (p : Point) : Bool =
  match b {
    MkBox lo hi ↦
      match lo {
        MkPoint lx ly ↦
          match hi {
            MkPoint hx hy ↦
              match p {
                MkPoint px py ↦ bool_and (axis_contains lx hx px) (axis_contains ly hy py)
              }
          }
      }
  }

pub fn intersect (b : Box) (c : Box) : Box =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          MkBox
                            (MkPoint (max blx clx) (max bly cly))
                            (MkPoint (min bhx chx) (min bhy chy))
                      }
                  }
              }
          }
      }
  }

pub fn hull (b : Box) (c : Box) : Box =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          MkBox
                            (MkPoint (min blx clx) (min bly cly))
                            (MkPoint (max bhx chx) (max bhy chy))
                      }
                  }
              }
          }
      }
  }

pub fn manhattan (p : Point) (q : Point) : Nat =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦ add (axis_distance px qx) (axis_distance py qy)
      }
  }

pub theorem intersect_sound
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains (intersect b c) p)
      → And (IsTrue (contains b p)) (IsTrue (contains c p)) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λh.
                                and_intro
                                  (IsTrue
                                    (bool_and
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)))
                                  (IsTrue
                                    (bool_and
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)))
                                  ((proof intro for bool_and)
                                    (axis_contains blx bhx px)
                                    (axis_contains bly bhy py)
                                    (and_fst
                                      (IsTrue (axis_contains blx bhx px))
                                      (IsTrue (axis_contains clx chx px))
                                      (axis_meet_sound
                                        blx
                                        bhx
                                        clx
                                        chx
                                        px
                                        ((proof left for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h)))
                                    (and_fst
                                      (IsTrue (axis_contains bly bhy py))
                                      (IsTrue (axis_contains cly chy py))
                                      (axis_meet_sound
                                        bly
                                        bhy
                                        cly
                                        chy
                                        py
                                        ((proof right for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h))))
                                  ((proof intro for bool_and)
                                    (axis_contains clx chx px)
                                    (axis_contains cly chy py)
                                    (and_snd
                                      (IsTrue (axis_contains blx bhx px))
                                      (IsTrue (axis_contains clx chx px))
                                      (axis_meet_sound
                                        blx
                                        bhx
                                        clx
                                        chx
                                        px
                                        ((proof left for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h)))
                                    (and_snd
                                      (IsTrue (axis_contains bly bhy py))
                                      (IsTrue (axis_contains cly chy py))
                                      (axis_meet_sound
                                        bly
                                        bhy
                                        cly
                                        chy
                                        py
                                        ((proof right for bool_and)
                                          (axis_contains (max blx clx) (min bhx chx) px)
                                          (axis_contains (max bly cly) (min bhy chy) py)
                                          h))))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem intersect_complete
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains b p) → IsTrue (contains c p) → IsTrue (contains (intersect b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhb.
                                λhc.
                                  (proof intro for bool_and)
                                    (axis_contains (max blx clx) (min bhx chx) px)
                                    (axis_contains (max bly cly) (min bhy chy) py)
                                    (axis_meet_complete
                                      blx
                                      bhx
                                      clx
                                      chx
                                      px
                                      ((proof left for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        hb)
                                      ((proof left for bool_and)
                                        (axis_contains clx chx px)
                                        (axis_contains cly chy py)
                                        hc))
                                    (axis_meet_complete
                                      bly
                                      bhy
                                      cly
                                      chy
                                      py
                                      ((proof right for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        hb)
                                      ((proof right for bool_and)
                                        (axis_contains clx chx px)
                                        (axis_contains cly chy py)
                                        hc))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem hull_contains_left
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains b p) → IsTrue (contains (hull b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhb.
                                (proof intro for bool_and)
                                  (axis_contains (min blx clx) (max bhx chx) px)
                                  (axis_contains (min bly cly) (max bhy chy) py)
                                  (axis_join_left
                                    blx
                                    bhx
                                    clx
                                    chx
                                    px
                                    ((proof left for bool_and)
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)
                                      hb))
                                  (axis_join_left
                                    bly
                                    bhy
                                    cly
                                    chy
                                    py
                                    ((proof right for bool_and)
                                      (axis_contains blx bhx px)
                                      (axis_contains bly bhy py)
                                      hb))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem hull_contains_right
      (b : Box) (c : Box) (p : Point)
    : IsTrue (contains c p) → IsTrue (contains (hull b c) p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λhc.
                                (proof intro for bool_and)
                                  (axis_contains (min blx clx) (max bhx chx) px)
                                  (axis_contains (min bly cly) (max bhy chy) py)
                                  (axis_join_right
                                    blx
                                    bhx
                                    clx
                                    chx
                                    px
                                    ((proof left for bool_and)
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)
                                      hc))
                                  (axis_join_right
                                    bly
                                    bhy
                                    cly
                                    chy
                                    py
                                    ((proof right for bool_and)
                                      (axis_contains clx chx px)
                                      (axis_contains cly chy py)
                                      hc))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem manhattan_symm (p : Point) (q : Point) : Equal Nat (manhattan p q) (manhattan q p) =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦
          trans
            Nat
            (add (axis_distance px qx) (axis_distance py qy))
            (add (axis_distance qx px) (axis_distance py qy))
            (add (axis_distance qx px) (axis_distance qy py))
            (cong
              Nat
              Nat
              (axis_distance px qx)
              (axis_distance qx px)
              (add_right (axis_distance py qy))
              (axis_distance_symm px qx))
            (cong
              Nat
              Nat
              (axis_distance py qy)
              (axis_distance qy py)
              (add (axis_distance qx px))
              (axis_distance_symm py qy))
      }
  }

pub theorem manhattan_self (p : Point) : Equal Nat (manhattan p p) Zero =
  match p {
    MkPoint px py ↦
      add_zero_zero
        (axis_distance px px)
        (axis_distance py py)
        (axis_distance_self px)
        (axis_distance_self py)
  }

fn axis_distance (a : Nat) (b : Nat) : Nat = add (sub a b) (sub b a)

theorem axis_meet_sound
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains (max l1 l2) (min h1 h2) x)
      → And (IsTrue (axis_contains l1 h1 x)) (IsTrue (axis_contains l2 h2 x)) =
  λh.
    and_intro
      (IsTrue (axis_contains l1 h1 x))
      (IsTrue (axis_contains l2 h2 x))
      ((proof intro for bool_and)
        (leq_nat l1 x)
        (leq_nat x h1)
        ((proof trans for leq_nat)
          l1
          (max l1 l2)
          x
          ((proof left_leq for max) l1 l2)
          ((proof left for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h))
        ((proof trans for leq_nat)
          x
          (min h1 h2)
          h1
          ((proof right for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h)
          ((proof leq_left for min) h1 h2)))
      ((proof intro for bool_and)
        (leq_nat l2 x)
        (leq_nat x h2)
        ((proof trans for leq_nat)
          l2
          (max l1 l2)
          x
          ((proof right_leq for max) l1 l2)
          ((proof left for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h))
        ((proof trans for leq_nat)
          x
          (min h1 h2)
          h2
          ((proof right for bool_and) (leq_nat (max l1 l2) x) (leq_nat x (min h1 h2)) h)
          ((proof leq_right for min) h1 h2)))

theorem axis_meet_complete
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l1 h1 x)
      → IsTrue (axis_contains l2 h2 x)
      → IsTrue (axis_contains (max l1 l2) (min h1 h2) x) =
  λa.
    λb.
      (proof intro for bool_and)
        (leq_nat (max l1 l2) x)
        (leq_nat x (min h1 h2))
        ((proof least for max)
          x
          l1
          l2
          ((proof left for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
          ((proof left for bool_and) (leq_nat l2 x) (leq_nat x h2) b))
        ((proof greatest for min)
          x
          h1
          h2
          ((proof right for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
          ((proof right for bool_and) (leq_nat l2 x) (leq_nat x h2) b))

theorem axis_join_left
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l1 h1 x) → IsTrue (axis_contains (min l1 l2) (max h1 h2) x) =
  λa.
    (proof intro for bool_and)
      (leq_nat (min l1 l2) x)
      (leq_nat x (max h1 h2))
      ((proof trans for leq_nat)
        (min l1 l2)
        l1
        x
        ((proof leq_left for min) l1 l2)
        ((proof left for bool_and) (leq_nat l1 x) (leq_nat x h1) a))
      ((proof trans for leq_nat)
        x
        h1
        (max h1 h2)
        ((proof right for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
        ((proof left_leq for max) h1 h2))

theorem axis_join_right
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_contains l2 h2 x) → IsTrue (axis_contains (min l1 l2) (max h1 h2) x) =
  λa.
    (proof intro for bool_and)
      (leq_nat (min l1 l2) x)
      (leq_nat x (max h1 h2))
      ((proof trans for leq_nat)
        (min l1 l2)
        l2
        x
        ((proof leq_right for min) l1 l2)
        ((proof left for bool_and) (leq_nat l2 x) (leq_nat x h2) a))
      ((proof trans for leq_nat)
        x
        h2
        (max h1 h2)
        ((proof right for bool_and) (leq_nat l2 x) (leq_nat x h2) a)
        ((proof right_leq for max) h1 h2))

fn add_right (y : Nat) (x : Nat) : Nat = add x y

theorem axis_distance_symm
      (a : Nat) (b : Nat)
    : Equal Nat (axis_distance a b) (axis_distance b a) =
  (proof comm for add) (sub a b) (sub b a)

theorem add_zero_zero
      (x : Nat) (y : Nat)
    : Equal Nat x Zero → Equal Nat y Zero → Equal Nat (add x y) Zero =
  λhx.
    λhy.
      trans
        Nat
        (add x y)
        (add Zero y)
        Zero
        (cong Nat Nat x Zero (add_right y) hx)
        (trans Nat (add Zero y) y Zero ((proof zero_l for add) y) hy)

theorem axis_distance_self (a : Nat) : Equal Nat (axis_distance a a) Zero =
  add_zero_zero
    (sub a a)
    (sub a a)
    ((proof self_is_zero for sub) a)
    ((proof self_is_zero for sub) a)
```

The metric extension uses saturating natural subtraction on each axis.
The triangle bound holds for every intermediate point, including a
strictly longer detour. Zero distance gives equality of both coordinates;
the two public theorems are supported by private per-axis proofs. No
additional numerical carrier or trusted arithmetic operation is needed.

```ken
pub theorem manhattan_triangle
      (p : Point) (q : Point) (r : Point)
    : IsTrue (leq_nat (manhattan p r) (add (manhattan p q) (manhattan q r))) =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦
          match r {
            MkPoint rx ry ↦
              let
                direct_distance : Nat = add (axis_distance px rx) (axis_distance py ry);
                axis_bounds : Nat =
                  add
                    (add (axis_distance px qx) (axis_distance qx rx))
                    (add (axis_distance py qy) (axis_distance qy ry));
                detour_distance : Nat =
                  add
                    (add (axis_distance px qx) (axis_distance py qy))
                    (add (axis_distance qx rx) (axis_distance qy ry))
              in
                leq_nat_eq_right
                  direct_distance
                  axis_bounds
                  detour_distance
                  (leq_nat_add_mono
                    (axis_distance px rx)
                    (add (axis_distance px qx) (axis_distance qx rx))
                    (axis_distance py ry)
                    (add (axis_distance py qy) (axis_distance qy ry))
                    (axis_distance_triangle px qx rx)
                    (axis_distance_triangle py qy ry))
                  (add_medial
                    (axis_distance px qx)
                    (axis_distance qx rx)
                    (axis_distance py qy)
                    (axis_distance qy ry))
          }
      }
  }

pub theorem manhattan_zero_equal
      (p : Point) (q : Point)
    : Equal Nat (manhattan p q) Zero → Equal Point p q =
  match p {
    MkPoint px py ↦
      match q {
        MkPoint qx qy ↦
          λh.
            let
              x_distance : Nat = axis_distance px qx;
              y_distance : Nat = axis_distance py qy
            in
              point_equal
                px
                qx
                py
                qy
                (axis_distance_zero px qx (add_zero_left x_distance y_distance h))
                (axis_distance_zero py qy (add_zero_right x_distance y_distance h))
      }
  }

theorem axis_distance_triangle
      (a : Nat) (b : Nat) (c : Nat)
    : IsTrue (leq_nat (axis_distance a c) (add (axis_distance a b) (axis_distance b c))) =
  leq_nat_eq_right
    (axis_distance a c)
    (add (add (sub a b) (sub b c)) (add (sub b a) (sub c b)))
    (add (axis_distance a b) (axis_distance b c))
    (leq_nat_add_mono
      (sub a c)
      (add (sub a b) (sub b c))
      (sub c a)
      (add (sub b a) (sub c b))
      (sub_triangle a b c)
      (leq_nat_eq_right
        (sub c a)
        (add (sub c b) (sub b a))
        (add (sub b a) (sub c b))
        (sub_triangle c b a)
        ((proof comm for add) (sub c b) (sub b a))))
    (add_medial (sub a b) (sub b c) (sub b a) (sub c b))

theorem sub_triangle
      (a : Nat) (b : Nat) (c : Nat)
    : IsTrue (leq_nat (sub a c) (add (sub a b) (sub b c))) =
  match c {
    Zero ↦ sub_add_bound a b;
    Suc c2 ↦
      match a {
        Zero ↦ Proved;
        Suc a2 ↦
          match b {
            Zero ↦ leq_nat_weaken_right (sub a2 c2) a2 (sub_leq a2 c2);
            Suc b2 ↦ sub_triangle a2 b2 c2
          }
      }
  }

theorem sub_add_bound (a : Nat) (b : Nat) : IsTrue (leq_nat a (add (sub a b) b)) =
  match b {
    Zero ↦ (proof refl for leq_nat) a;
    Suc b2 ↦
      match a {
        Zero ↦ Proved;
        Suc a2 ↦ sub_add_bound a2 b2
      }
  }

theorem sub_leq (a : Nat) (c : Nat) : IsTrue (leq_nat (sub a c) a) =
  match c {
    Zero ↦ (proof refl for leq_nat) a;
    Suc c2 ↦
      match a {
        Zero ↦ Proved;
        Suc a2 ↦ leq_nat_weaken_right (sub a2 c2) a2 (sub_leq a2 c2)
      }
  }

theorem leq_nat_add_mono_left
      (x : Nat) (x2 : Nat) (y : Nat)
    : IsTrue (leq_nat x x2) → IsTrue (leq_nat (add x y) (add x2 y)) =
  match y {
    Zero ↦ λh. h;
    Suc y2 ↦ λh. leq_nat_add_mono_left x x2 y2 h
  }

theorem leq_nat_add_mono_right
      (x : Nat) (y : Nat)
    : (y2 : Nat) → IsTrue (leq_nat y y2) → IsTrue (leq_nat (add x y) (add x y2)) =
  match y {
    Zero ↦ λy2. λh. leq_nat_add_left_bound x y2;
    Suc y1 ↦
      λy2.
        match y2 {
          Zero ↦ λh. absurd h;
          Suc y3 ↦ λh. leq_nat_add_mono_right x y1 y3 h
        }
  }

theorem leq_nat_add_mono
      (x : Nat) (x2 : Nat) (y : Nat) (y2 : Nat)
    : IsTrue (leq_nat x x2) → IsTrue (leq_nat y y2) → IsTrue (leq_nat (add x y) (add x2 y2)) =
  λhx.
    λhy.
      (proof trans for leq_nat)
        (add x y)
        (add x2 y)
        (add x2 y2)
        (leq_nat_add_mono_left x x2 y hx)
        (leq_nat_add_mono_right x2 y y2 hy)

theorem leq_nat_eq_right
      (x : Nat) (y : Nat) (z : Nat)
    : IsTrue (leq_nat x y) → Equal Nat y z → IsTrue (leq_nat x z) =
  λh.
    λe.
      trans
        Bool
        (leq_nat x z)
        (leq_nat x y)
        True
        (sym Bool (leq_nat x y) (leq_nat x z) (cong Nat Bool y z (leq_nat x) e))
        h

fn add_inner (a : Nat) (d : Nat) (t : Nat) : Nat = add (add a t) d

theorem add_medial
      (a : Nat) (b : Nat) (c : Nat) (d : Nat)
    : Equal Nat (add (add a b) (add c d)) (add (add a c) (add b d)) =
  trans
    Nat
    (add (add a b) (add c d))
    (add (add (add a b) c) d)
    (add (add a c) (add b d))
    ((proof assoc for add) (add a b) c d)
    (trans
      Nat
      (add (add (add a b) c) d)
      (add (add a (add b c)) d)
      (add (add a c) (add b d))
      (cong
        Nat
        Nat
        (add (add a b) c)
        (add a (add b c))
        (add_right d)
        (sym Nat (add a (add b c)) (add (add a b) c) ((proof assoc for add) a b c)))
      (trans
        Nat
        (add (add a (add b c)) d)
        (add (add a (add c b)) d)
        (add (add a c) (add b d))
        (cong Nat Nat (add b c) (add c b) (add_inner a d) ((proof comm for add) b c))
        (trans
          Nat
          (add (add a (add c b)) d)
          (add (add (add a c) b) d)
          (add (add a c) (add b d))
          (cong
            Nat
            Nat
            (add a (add c b))
            (add (add a c) b)
            (add_right d)
            ((proof assoc for add) a c b))
          (sym
            Nat
            (add (add a c) (add b d))
            (add (add (add a c) b) d)
            ((proof assoc for add) (add a c) b d)))))

theorem add_zero_left (x : Nat) (y : Nat) : Equal Nat (add x y) Zero → Equal Nat x Zero =
  match y {
    Zero ↦ λh. h;
    Suc y2 ↦ λh. absurd h
  }

theorem add_zero_right (x : Nat) (y : Nat) : Equal Nat (add x y) Zero → Equal Nat y Zero =
  match y {
    Zero ↦ λh. Proved;
    Suc y2 ↦ λh. absurd h
  }

theorem axis_distance_zero
      (a : Nat)
    : (b : Nat) → Equal Nat (axis_distance a b) Zero → Equal Nat a b =
  match a {
    Zero ↦
      λb.
        match b {
          Zero ↦ λh. Proved;
          Suc b2 ↦ λh. absurd h
        };
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦ λh. cong Nat Nat a2 b2 Suc (axis_distance_zero a2 b2 h)
        }
  }

fn point_with_x (x : Nat) (y : Nat) : Point = MkPoint x y

fn point_with_y (y : Nat) (x : Nat) : Point = MkPoint x y

theorem point_equal
      (x1 : Nat) (x2 : Nat) (y1 : Nat) (y2 : Nat)
    : Equal Nat x1 x2 → Equal Nat y1 y2 → Equal Point (MkPoint x1 y1) (MkPoint x2 y2) =
  λhx.
    λhy.
      trans
        Point
        (MkPoint x1 y1)
        (MkPoint x2 y1)
        (MkPoint x2 y2)
        (cong Nat Point x1 x2 (point_with_y y1) hx)
        (cong Nat Point y1 y2 (point_with_x x2) hy)
```

```ken
pub fn inside (b : Box) (c : Box) : Bool =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          bool_and (axis_within blx bhx clx chx) (axis_within bly bhy cly chy)
                      }
                  }
              }
          }
      }
  }

pub theorem inside_sound
      (b : Box) (c : Box) (p : Point)
    : IsTrue (inside b c) → IsTrue (contains b p) → IsTrue (contains c p) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match p {
                            MkPoint px py ↦
                              λw.
                                λh.
                                  (proof intro for bool_and)
                                    (axis_contains clx chx px)
                                    (axis_contains cly chy py)
                                    (axis_within_sound
                                      blx
                                      bhx
                                      clx
                                      chx
                                      px
                                      ((proof left for bool_and)
                                        (axis_within blx bhx clx chx)
                                        (axis_within bly bhy cly chy)
                                        w)
                                      ((proof left for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        h))
                                    (axis_within_sound
                                      bly
                                      bhy
                                      cly
                                      chy
                                      py
                                      ((proof right for bool_and)
                                        (axis_within blx bhx clx chx)
                                        (axis_within bly bhy cly chy)
                                        w)
                                      ((proof right for bool_and)
                                        (axis_contains blx bhx px)
                                        (axis_contains bly bhy py)
                                        h))
                          }
                      }
                  }
              }
          }
      }
  }

pub theorem inside_refl (b : Box) : IsTrue (inside b b) =
  match b {
    MkBox lo hi ↦
      match lo {
        MkPoint lx ly ↦
          match hi {
            MkPoint hx hy ↦
              (proof intro for bool_and)
                (axis_within lx hx lx hx)
                (axis_within ly hy ly hy)
                (axis_within_refl lx hx)
                (axis_within_refl ly hy)
          }
      }
  }

pub theorem intersect_inside_left (b : Box) (c : Box) : IsTrue (inside (intersect b c) b) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          (proof intro for bool_and)
                            (axis_within (max blx clx) (min bhx chx) blx bhx)
                            (axis_within (max bly cly) (min bhy chy) bly bhy)
                            ((proof intro for bool_and)
                              (leq_nat blx (max blx clx))
                              (leq_nat (min bhx chx) bhx)
                              ((proof left_leq for max) blx clx)
                              ((proof leq_left for min) bhx chx))
                            ((proof intro for bool_and)
                              (leq_nat bly (max bly cly))
                              (leq_nat (min bhy chy) bhy)
                              ((proof left_leq for max) bly cly)
                              ((proof leq_left for min) bhy chy))
                      }
                  }
              }
          }
      }
  }

pub theorem intersect_inside_right (b : Box) (c : Box) : IsTrue (inside (intersect b c) c) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          (proof intro for bool_and)
                            (axis_within (max blx clx) (min bhx chx) clx chx)
                            (axis_within (max bly cly) (min bhy chy) cly chy)
                            ((proof intro for bool_and)
                              (leq_nat clx (max blx clx))
                              (leq_nat (min bhx chx) chx)
                              ((proof right_leq for max) blx clx)
                              ((proof leq_right for min) bhx chx))
                            ((proof intro for bool_and)
                              (leq_nat cly (max bly cly))
                              (leq_nat (min bhy chy) chy)
                              ((proof right_leq for max) bly cly)
                              ((proof leq_right for min) bhy chy))
                      }
                  }
              }
          }
      }
  }

pub theorem inside_hull_left (b : Box) (c : Box) : IsTrue (inside b (hull b c)) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          (proof intro for bool_and)
                            (axis_within blx bhx (min blx clx) (max bhx chx))
                            (axis_within bly bhy (min bly cly) (max bhy chy))
                            ((proof intro for bool_and)
                              (leq_nat (min blx clx) blx)
                              (leq_nat bhx (max bhx chx))
                              ((proof leq_left for min) blx clx)
                              ((proof left_leq for max) bhx chx))
                            ((proof intro for bool_and)
                              (leq_nat (min bly cly) bly)
                              (leq_nat bhy (max bhy chy))
                              ((proof leq_left for min) bly cly)
                              ((proof left_leq for max) bhy chy))
                      }
                  }
              }
          }
      }
  }

pub theorem inside_hull_right (b : Box) (c : Box) : IsTrue (inside c (hull b c)) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          (proof intro for bool_and)
                            (axis_within clx chx (min blx clx) (max bhx chx))
                            (axis_within cly chy (min bly cly) (max bhy chy))
                            ((proof intro for bool_and)
                              (leq_nat (min blx clx) clx)
                              (leq_nat chx (max bhx chx))
                              ((proof leq_right for min) blx clx)
                              ((proof right_leq for max) bhx chx))
                            ((proof intro for bool_and)
                              (leq_nat (min bly cly) cly)
                              (leq_nat chy (max bhy chy))
                              ((proof leq_right for min) bly cly)
                              ((proof right_leq for max) bhy chy))
                      }
                  }
              }
          }
      }
  }

pub theorem inside_complete
      (b : Box) (c : Box) (q : Point)
    : IsTrue (contains b q)
      → ((p : Point) → IsTrue (contains b p) → IsTrue (contains c p))
      → IsTrue (inside b c) =
  match b {
    MkBox blo bhi ↦
      match c {
        MkBox clo chi ↦
          match blo {
            MkPoint blx bly ↦
              match bhi {
                MkPoint bhx bhy ↦
                  match clo {
                    MkPoint clx cly ↦
                      match chi {
                        MkPoint chx chy ↦
                          match q {
                            MkPoint qx qy ↦
                              λhq.
                                λall.
                                  let
                                    qxin : IsTrue (axis_contains blx bhx qx) =
                                      (proof left for bool_and)
                                        (axis_contains blx bhx qx)
                                        (axis_contains bly bhy qy)
                                        hq;
                                    qyin : IsTrue (axis_contains bly bhy qy) =
                                      (proof right for bool_and)
                                        (axis_contains blx bhx qx)
                                        (axis_contains bly bhy qy)
                                        hq;
                                    low : IsTrue
                                      (contains
                                        (MkBox (MkPoint clx cly) (MkPoint chx chy))
                                        (MkPoint blx bly)) =
                                      all
                                        (MkPoint blx bly)
                                        ((proof intro for bool_and)
                                          (axis_contains blx bhx blx)
                                          (axis_contains bly bhy bly)
                                          (axis_corner_low blx bhx qx qxin)
                                          (axis_corner_low bly bhy qy qyin));
                                    high : IsTrue
                                      (contains
                                        (MkBox (MkPoint clx cly) (MkPoint chx chy))
                                        (MkPoint bhx bhy)) =
                                      all
                                        (MkPoint bhx bhy)
                                        ((proof intro for bool_and)
                                          (axis_contains blx bhx bhx)
                                          (axis_contains bly bhy bhy)
                                          (axis_corner_high blx bhx qx qxin)
                                          (axis_corner_high bly bhy qy qyin))
                                  in
                                    (proof intro for bool_and)
                                      (axis_within blx bhx clx chx)
                                      (axis_within bly bhy cly chy)
                                      ((proof intro for bool_and)
                                        (leq_nat clx blx)
                                        (leq_nat bhx chx)
                                        ((proof left for bool_and)
                                          (leq_nat clx blx)
                                          (leq_nat blx chx)
                                          ((proof left for bool_and)
                                            (axis_contains clx chx blx)
                                            (axis_contains cly chy bly)
                                            low))
                                        ((proof right for bool_and)
                                          (leq_nat clx bhx)
                                          (leq_nat bhx chx)
                                          ((proof left for bool_and)
                                            (axis_contains clx chx bhx)
                                            (axis_contains cly chy bhy)
                                            high)))
                                      ((proof intro for bool_and)
                                        (leq_nat cly bly)
                                        (leq_nat bhy chy)
                                        ((proof left for bool_and)
                                          (leq_nat cly bly)
                                          (leq_nat bly chy)
                                          ((proof right for bool_and)
                                            (axis_contains clx chx blx)
                                            (axis_contains cly chy bly)
                                            low))
                                        ((proof right for bool_and)
                                          (leq_nat cly bhy)
                                          (leq_nat bhy chy)
                                          ((proof right for bool_and)
                                            (axis_contains clx chx bhx)
                                            (axis_contains cly chy bhy)
                                            high)))
                          }
                      }
                  }
              }
          }
      }
  }

fn axis_within (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) : Bool =
  bool_and (leq_nat l2 l1) (leq_nat h1 h2)

theorem axis_within_sound
      (l1 : Nat) (h1 : Nat) (l2 : Nat) (h2 : Nat) (x : Nat)
    : IsTrue (axis_within l1 h1 l2 h2)
      → IsTrue (axis_contains l1 h1 x)
      → IsTrue (axis_contains l2 h2 x) =
  λw.
    λa.
      (proof intro for bool_and)
        (leq_nat l2 x)
        (leq_nat x h2)
        ((proof trans for leq_nat)
          l2
          l1
          x
          ((proof left for bool_and) (leq_nat l2 l1) (leq_nat h1 h2) w)
          ((proof left for bool_and) (leq_nat l1 x) (leq_nat x h1) a))
        ((proof trans for leq_nat)
          x
          h1
          h2
          ((proof right for bool_and) (leq_nat l1 x) (leq_nat x h1) a)
          ((proof right for bool_and) (leq_nat l2 l1) (leq_nat h1 h2) w))

theorem axis_within_refl (l : Nat) (h : Nat) : IsTrue (axis_within l h l h) =
  (proof intro for bool_and)
    (leq_nat l l)
    (leq_nat h h)
    ((proof refl for leq_nat) l)
    ((proof refl for leq_nat) h)

theorem axis_corner_low
      (l : Nat) (h : Nat) (x : Nat)
    : IsTrue (axis_contains l h x) → IsTrue (axis_contains l h l) =
  λa.
    (proof intro for bool_and)
      (leq_nat l l)
      (leq_nat l h)
      ((proof refl for leq_nat) l)
      ((proof trans for leq_nat)
        l
        x
        h
        ((proof left for bool_and) (leq_nat l x) (leq_nat x h) a)
        ((proof right for bool_and) (leq_nat l x) (leq_nat x h) a))

theorem axis_corner_high
      (l : Nat) (h : Nat) (x : Nat)
    : IsTrue (axis_contains l h x) → IsTrue (axis_contains l h h) =
  λa.
    (proof intro for bool_and)
      (leq_nat l h)
      (leq_nat h h)
      ((proof trans for leq_nat)
        l
        x
        h
        ((proof left for bool_and) (leq_nat l x) (leq_nat x h) a)
        ((proof right for bool_and) (leq_nat l x) (leq_nat x h) a))
      ((proof refl for leq_nat) h)
```

## 3. Using it

Constructing a box does not require a validity premise. Both endpoints
belong to this example's closed unit square:

```ken example
const box_lower : Point = MkPoint Zero Zero

const box_upper : Point = MkPoint (Suc Zero) (Suc Zero)

const unit_box : Box = MkBox box_lower box_upper

theorem unit_box_includes_upper : Equal Bool (contains unit_box box_upper) True = Proved
```

Intersecting separated boxes still produces a `Box`: its inverted corners
express the empty result. `manhattan` on points `(1,4)` and `(3,1)` yields
five, with a contribution from each axis. The checked
[seed client](../../../../conformance/stdlib/geometry/seed-nat-box.md)
constructs these cases independently of the operations under test.

A detour from `(0,0)` through `(3,2)` to `(1,1)` is strictly longer
than the direct route: its legs sum to eight, while the direct distance
is two. `manhattan_triangle` supplies the general bound, not just the
closed observation. From zero distance, `manhattan_zero_equal` lets a
client recover equality of points:

```ken example
const metric_origin : Point = MkPoint Zero Zero

theorem metric_origin_self : Equal Point metric_origin metric_origin =
  manhattan_zero_equal metric_origin metric_origin (manhattan_self metric_origin)
```

The checked
[metric seed](../../../../conformance/stdlib/geometry/seed-manhattan-metric.md)
includes both strict and tight detours and separately rejects the false
reverse bound and a zero distance between distinct points.

`inside` compares the corners of two boxes, not just their sets of points.
It recognizes a nested box and an inverted one whose corners still fit
inside the outer box. The
[containment seed](../../../../conformance/stdlib/geometry/seed-box-containment.md)
checks both, intersections inside each input, and both inputs inside their
hull. It also refuses an inner box's claim to fit inside a strictly
smaller box.

## 4. Laws & proofs

`intersect_sound` projects both coordinate witnesses from an intersection,
then transports each lower and upper comparison through the corresponding
`max` and `min` bounds. `intersect_complete` combines membership in both
inputs using the existing checked tightness laws `max::least` and
`min::greatest`; it never clones those proofs. The two hull laws separately
transport membership from the left and right inputs to their wider bounds.

`manhattan_symm` commutes each axis distance's two subtraction terms and
lifts those equalities through addition with `cong` and `trans`.
`manhattan_self` uses the existing checked subtraction self-zero proof on
each axis. `manhattan_triangle` proves each axis's saturating-subtraction
triangle by induction, adds the two bounds monotonically, and reorders
addition to match the distance definition. `manhattan_zero_equal` splits
a zero sum into zero per-axis distances, derives equality of both
coordinates by induction, and lifts them into point equality with `cong`.
The private `axis_meet_*` and `axis_join_*` lemmas keep the coordinate
arguments of the box laws explicit. All fifteen public statements are
generic in their boxes or points, and none identifies boxes by their
sets of members.

`inside_sound` transports point membership along the four corner comparisons
of `inside`; `inside_refl` uses checked order reflexivity, including for
inverted boxes. The two intersection laws use the respective `max` and `min`
bounds, while the hull laws use the opposite extrema. `inside_complete`
requires a point in the inner box before deriving its two corner witnesses;
pointwise inclusion alone would be vacuous for an inverted inner box. The
private `axis_within` and per-axis lemmas keep these distinctions explicit.

## 5. Design notes

No validity bit or special empty constructor is needed. A box with inverted
corners remains representable and the membership predicate refuses every
point on its inverted axis. `Eq Box` compares corners structurally rather
than quotienting by membership. Hull computes a coordinatewise enclosure,
including the corners of an empty input; it does not claim a least point-set
union in the presence of differently represented empty boxes. Manhattan
distance is exact and unsigned; its triangle inequality and zero-distance
identity are proved. Corner-wise `inside` is directional and sufficient for
point-set inclusion, but an empty inner box can be outside a second box by
its corners despite having no points. Its converse law therefore requires
a member of the inner box. Area and signed orientation remain outside this
interface.

## 6. References

- [Axis-aligned bounding box](https://en.wikipedia.org/wiki/Minimum_bounding_box)
  — Wikipedia — geometric orientation for coordinatewise extrema.
- [Taxicab geometry](https://en.wikipedia.org/wiki/Taxicab_geometry)
  — Wikipedia — orientation for the sum of coordinate distances.
- [Natural order](../../Data/Numeric/Nat/Order.ken.md) — the checked `min`,
  `max`, `sub` and order-law provider used by the eight laws.

## 7. Trust & derivation

1. **Contract.** [Exact geometry, §§1–3](../../../../spec/50-stdlib/67-geometry.md)
   specifies the six operations and fifteen public proof types.
2. **Public API.** `Point`, `MkPoint`, `Box`, `MkBox`, `axis_contains`,
   `contains`, `intersect`, `hull`, `manhattan`, `intersect_sound`,
   `intersect_complete`, `hull_contains_left`, `hull_contains_right`,
   `manhattan_symm`, `manhattan_self`, `manhattan_triangle`,
   `manhattan_zero_equal`, `inside`, `inside_sound`, `inside_refl`,
   `intersect_inside_left`, `intersect_inside_right`, `inside_hull_left`,
   `inside_hull_right`, and `inside_complete`. `axis_within`, per-axis
   lemmas and proof transport helpers remain private.
3. **Source map.** Definition contains the constructors and operations,
   followed by the checked laws and private scaffolding. Using it points to
   closed examples and generic metric and containment use; Laws & proofs
   maps each theorem to its proof strategy.
4. **Derivation.** `Nat`, `Bool` and the ambient conjunction come from the
   compiler floor and checked classes. Arithmetic reuses the canonical
   `Data.Numeric.Nat.Arithmetic` addition and checked
   `Data.Numeric.Nat.Order` bounds and saturating subtraction. The
   metric proofs also use existing checked equality transport. The
   source declares no `Axiom` or new primitive.
5. **Trust.** Loading this package adds zero trusted `GlobalId`s beyond the
   already-loaded provider closure; that closure may itself carry inherited
   trust. A targeted roots-loader test measures the before/after identity sets.
6. **Validation.** The
   [box seed](../../../../conformance/stdlib/geometry/seed-nat-box.md)
   covers the six original laws, while the
   [metric seed](../../../../conformance/stdlib/geometry/seed-manhattan-metric.md)
   gives strict and tight detours, a generic identity-law client, closed
   instances of both laws, and false distance/inequality assertions.
   The [seed](../../../../conformance/stdlib/geometry/seed-box-containment.md)
   checks nested, intersecting, hull, self and inverted boxes, with the
   false inward claim separately refused. All three supplement rather
   than replace the checked law bodies.
