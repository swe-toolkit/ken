# Exact non-negative rational values

`Rational` carries an arbitrary natural numerator and a positive denominator.
Equivalence, arithmetic and order compare values without normalizing their
representations.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A natural numerator and a syntactically positive denominator suffice to
represent every non-negative rational. The constructor preserves the supplied
spelling; equality of values is instead stated by equality of cross products.
No normal form, quotient, rounding, or floating comparison is required.
`rational_sub` truncates at zero: subtracting a larger value never introduces a
signed result. `rational_min` and `rational_max` select one stored rational by
exact value order; their six checked bounds support interval hull and
intersection.

## 2. Definition

The second constructor field is the denominator's predecessor: `MkRational n k`
represents `n/(Suc k)`. All exported calculations preserve that positivity.
Min and max compare exact values and return an original operand; neither
normalizes its spelling. Subtraction cross-multiplies two positive denominators,
subtracts the numerators with Nat's truncated `sub`, and retains a positive
product denominator.

```ken
import Core.Logic.Transport (sym, trans, cong)

import Core.Classes.LawfulClasses (IsTrue, bool_or, leq_nat)

import Data.Numeric.Nat.Arithmetic
  (add, mul, mul_add_distrib_l, mul_add_distrib_r, add_cancel_r)

import Data.Numeric.Nat.Order
  (sub, lt_nat, leq_nat_mul_right_mono, leq_nat_mul_right_cancel, mul_sub_distrib_l)

pub data Rational : Type where {
  MkRational : (num : Nat) → (den_tail : Nat) → Rational
}

export MkRational

pub fn numerator (x : Rational) : Nat =
  match x {
    MkRational n d ↦ n
  }

pub fn denominator (x : Rational) : Nat =
  match x {
    MkRational n d ↦ Suc d
  }

pub const rational_zero : Rational = MkRational Zero Zero

pub const rational_one : Rational = MkRational (Suc Zero) Zero

pub fn rational_of_nat (n : Nat) : Rational = MkRational n Zero

pub fn rational_equiv (x : Rational) (y : Rational) : Prop =
  Equal Nat (mul (numerator x) (denominator y)) (mul (numerator y) (denominator x))

pub proof positive for denominator
      (x : Rational)
    : Equal Bool (lt_nat Zero (denominator x)) True =
  match x {
    MkRational n d ↦
      match d {
        Zero ↦ Proved;
        Suc tail ↦ Proved
      }
  }

pub proof refl for rational_equiv (x : Rational) : rational_equiv x x = Refl

pub proof sym for rational_equiv
      (x : Rational) (y : Rational) (h : rational_equiv x y)
    : rational_equiv y x =
  sym Nat (mul (numerator x) (denominator y)) (mul (numerator y) (denominator x)) h

pub proof trans for rational_equiv
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv x y → rational_equiv y z → rational_equiv x z =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          match z {
            MkRational e bz ↦
              λhxy.
                λhyz.
                  ((proof cancel_r for mul)
                    (mul a (Suc bz))
                    (mul e (Suc bx))
                    by
                    (trans
                      Nat
                      (mul (mul a (Suc bz)) (Suc by))
                      (mul (mul a (Suc by)) (Suc bz))
                      (mul (mul e (Suc bx)) (Suc by))
                      (sym
                        Nat
                        (mul (mul a (Suc by)) (Suc bz))
                        (mul (mul a (Suc bz)) (Suc by))
                        (mul_swap_tail a (Suc by) (Suc bz)))
                      (trans
                        Nat
                        (mul (mul a (Suc by)) (Suc bz))
                        (mul (mul c (Suc bx)) (Suc bz))
                        (mul (mul e (Suc bx)) (Suc by))
                        (cong
                          Nat
                          Nat
                          (mul a (Suc by))
                          (mul c (Suc bx))
                          (λv. mul v (Suc bz))
                          hxy)
                        (trans
                          Nat
                          (mul (mul c (Suc bx)) (Suc bz))
                          (mul (mul c (Suc bz)) (Suc bx))
                          (mul (mul e (Suc bx)) (Suc by))
                          (mul_swap_tail c (Suc bx) (Suc bz))
                          (trans
                            Nat
                            (mul (mul c (Suc bz)) (Suc bx))
                            (mul (mul e (Suc by)) (Suc bx))
                            (mul (mul e (Suc bx)) (Suc by))
                            (cong
                              Nat
                              Nat
                              (mul c (Suc bz))
                              (mul e (Suc by))
                              (λv. mul v (Suc bx))
                              hyz)
                            (mul_swap_tail e (Suc by) (Suc bx)))))))
          }
      }
  }

pub fn rational_add (x : Rational) (y : Rational) : Rational =
  match x {
    MkRational nx dx ↦
      match y {
        MkRational ny dy ↦
          MkRational (add (mul nx (Suc dy)) (mul ny (Suc dx))) (positive_product_tail dx dy)
      }
  }

pub fn rational_sub (x : Rational) (y : Rational) : Rational =
  match x {
    MkRational nx dx ↦
      match y {
        MkRational ny dy ↦
          MkRational (sub (mul nx (Suc dy)) (mul ny (Suc dx))) (positive_product_tail dx dy)
      }
  }

pub fn rational_mul (x : Rational) (y : Rational) : Rational =
  match x {
    MkRational nx dx ↦
      match y {
        MkRational ny dy ↦ MkRational (mul nx ny) (positive_product_tail dx dy)
      }
  }

pub fn rational_leq (x : Rational) (y : Rational) : Bool =
  leq_nat (mul (numerator x) (denominator y)) (mul (numerator y) (denominator x))

pub proof denominator_product for rational_add
      (x : Rational) (y : Rational)
    : Equal Nat (denominator (rational_add x y)) (mul (denominator x) (denominator y)) =
  match x {
    MkRational nx dx ↦
      match y {
        MkRational ny dy ↦ positive_product_factor dx dy
      }
  }

pub proof denominator_product for rational_mul
      (x : Rational) (y : Rational)
    : Equal Nat (denominator (rational_mul x y)) (mul (denominator x) (denominator y)) =
  match x {
    MkRational nx dx ↦
      match y {
        MkRational ny dy ↦ positive_product_factor dx dy
      }
  }

pub proof respect for rational_add
      (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
    : rational_equiv x xp
      → rational_equiv y yp
      → rational_equiv (rational_add x y) (rational_add xp yp) =
  match x {
    MkRational a bx ↦
      match xp {
        MkRational ap bxp ↦
          match y {
            MkRational c by ↦
              match yp {
                MkRational cp byp ↦
                  λha.
                    λhc. add_cross_respect a (Suc bx) c (Suc by) ap (Suc bxp) cp (Suc byp) ha hc
              }
          }
      }
  }

pub proof respect for rational_mul
      (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
    : rational_equiv x xp
      → rational_equiv y yp
      → rational_equiv (rational_mul x y) (rational_mul xp yp) =
  match x {
    MkRational a bx ↦
      match xp {
        MkRational ap bxp ↦
          match y {
            MkRational c by ↦
              match yp {
                MkRational cp byp ↦
                  λha.
                    λhc. mul_cross_respect a (Suc bx) c (Suc by) ap (Suc bxp) cp (Suc byp) ha hc
              }
          }
      }
  }

pub proof respect for rational_leq
      (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
    : rational_equiv x xp
      → rational_equiv y yp
      → Equal Bool (rational_leq x y) (rational_leq xp yp) =
  match x {
    MkRational a bx ↦
      match xp {
        MkRational ap bxp ↦
          match y {
            MkRational c by ↦
              match yp {
                MkRational cp byp ↦ λhx. λhy. leq_cross_respect a bx c by ap bxp cp byp hx hy
              }
          }
      }
  }

pub proof comm for rational_add
      (x : Rational) (y : Rational)
    : rational_equiv (rational_add x y) (rational_add y x) =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          mul_cong2
            (add (mul a (Suc by)) (mul c (Suc bx)))
            (add (mul c (Suc bx)) (mul a (Suc by)))
            (mul (Suc by) (Suc bx))
            (mul (Suc bx) (Suc by))
            ((proof comm for add) (mul a (Suc by)) (mul c (Suc bx)))
            ((proof comm for mul) (Suc by) (Suc bx))
      }
  }

pub proof assoc for rational_add
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv (rational_add x (rational_add y z)) (rational_add (rational_add x y) z) =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          match z {
            MkRational e bz ↦
              mul_cong2
                (add
                  (mul a (mul (Suc by) (Suc bz)))
                  (mul (add (mul c (Suc bz)) (mul e (Suc by))) (Suc bx)))
                (add
                  (mul (add (mul a (Suc by)) (mul c (Suc bx))) (Suc bz))
                  (mul e (mul (Suc bx) (Suc by))))
                (mul (mul (Suc bx) (Suc by)) (Suc bz))
                (mul (Suc bx) (mul (Suc by) (Suc bz)))
                (add_assoc_cross a c e (Suc bx) (Suc by) (Suc bz))
                (sym
                  Nat
                  (mul (Suc bx) (mul (Suc by) (Suc bz)))
                  (mul (mul (Suc bx) (Suc by)) (Suc bz))
                  ((proof assoc for mul) (Suc bx) (Suc by) (Suc bz)))
          }
      }
  }

pub proof zero_l for rational_add
      (x : Rational)
    : rational_equiv (rational_add rational_zero x) x =
  match x {
    MkRational n d ↦
      let
        n_zero : Equal Nat (add (mul Zero (Suc d)) (mul n (Suc Zero))) n =
          trans
            Nat
            (add (mul Zero (Suc d)) (mul n (Suc Zero)))
            (add Zero (mul n (Suc Zero)))
            n
            (cong
              Nat
              Nat
              (mul Zero (Suc d))
              Zero
              (λv. add v (mul n (Suc Zero)))
              ((proof zero_l for mul) (Suc d)))
            (trans
              Nat
              (add Zero (mul n (Suc Zero)))
              (mul n (Suc Zero))
              n
              ((proof zero_l for add) (mul n (Suc Zero)))
              ((proof one_r for mul) n));
        d_zero : Equal Nat (positive_product_tail Zero d) d = (proof one_l for mul) d
      in
        trans
          Nat
          (mul (add (mul Zero (Suc d)) (mul n (Suc Zero))) (Suc d))
          (mul n (Suc d))
          (mul n (Suc (positive_product_tail Zero d)))
          (cong
            Nat
            Nat
            (add (mul Zero (Suc d)) (mul n (Suc Zero)))
            n
            (λv. mul v (Suc d))
            n_zero)
          (cong
            Nat
            Nat
            d
            (positive_product_tail Zero d)
            (λv. mul n (Suc v))
            (sym Nat (positive_product_tail Zero d) d d_zero))
  }

pub proof zero_r for rational_add
      (x : Rational)
    : rational_equiv (rational_add x rational_zero) x =
  match x {
    MkRational n d ↦
      let
        n_zero : Equal Nat (add (mul n (Suc Zero)) (mul Zero (Suc d))) n =
          trans
            Nat
            (add (mul n (Suc Zero)) (mul Zero (Suc d)))
            (add n (mul Zero (Suc d)))
            n
            (cong
              Nat
              Nat
              (mul n (Suc Zero))
              n
              (λv. add v (mul Zero (Suc d)))
              ((proof one_r for mul) n))
            (cong
              Nat
              Nat
              (mul Zero (Suc d))
              Zero
              (λv. add n v)
              ((proof zero_l for mul) (Suc d)));
        d_zero : Equal Nat (positive_product_tail d Zero) d = (proof zero_l for add) d
      in
        trans
          Nat
          (mul (add (mul n (Suc Zero)) (mul Zero (Suc d))) (Suc d))
          (mul n (Suc d))
          (mul n (Suc (positive_product_tail d Zero)))
          (cong
            Nat
            Nat
            (add (mul n (Suc Zero)) (mul Zero (Suc d)))
            n
            (λv. mul v (Suc d))
            n_zero)
          (cong
            Nat
            Nat
            d
            (positive_product_tail d Zero)
            (λv. mul n (Suc v))
            (sym Nat (positive_product_tail d Zero) d d_zero))
  }

pub proof comm for rational_mul
      (x : Rational) (y : Rational)
    : rational_equiv (rational_mul x y) (rational_mul y x) =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          mul_cong2
            (mul a c)
            (mul c a)
            (mul (Suc by) (Suc bx))
            (mul (Suc bx) (Suc by))
            ((proof comm for mul) a c)
            ((proof comm for mul) (Suc by) (Suc bx))
      }
  }

pub proof assoc for rational_mul
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv (rational_mul x (rational_mul y z)) (rational_mul (rational_mul x y) z) =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          match z {
            MkRational e bz ↦
              mul_cong2
                (mul a (mul c e))
                (mul (mul a c) e)
                (mul (mul (Suc bx) (Suc by)) (Suc bz))
                (mul (Suc bx) (mul (Suc by) (Suc bz)))
                ((proof assoc for mul) a c e)
                (sym
                  Nat
                  (mul (Suc bx) (mul (Suc by) (Suc bz)))
                  (mul (mul (Suc bx) (Suc by)) (Suc bz))
                  ((proof assoc for mul) (Suc bx) (Suc by) (Suc bz)))
          }
      }
  }

pub proof one_l for rational_mul
      (x : Rational)
    : rational_equiv (rational_mul rational_one x) x =
  match x {
    MkRational n d ↦
      let
        n_one : Equal Nat (mul (Suc Zero) n) n = (proof one_l for mul) n;
        d_one : Equal Nat (positive_product_tail Zero d) d = (proof one_l for mul) d
      in
        trans
          Nat
          (mul (mul (Suc Zero) n) (Suc d))
          (mul n (Suc d))
          (mul n (Suc (positive_product_tail Zero d)))
          (cong Nat Nat (mul (Suc Zero) n) n (λv. mul v (Suc d)) n_one)
          (cong
            Nat
            Nat
            d
            (positive_product_tail Zero d)
            (λv. mul n (Suc v))
            (sym Nat (positive_product_tail Zero d) d d_one))
  }

pub proof one_r for rational_mul
      (x : Rational)
    : rational_equiv (rational_mul x rational_one) x =
  match x {
    MkRational n d ↦
      let
        n_one : Equal Nat (mul n (Suc Zero)) n = (proof one_r for mul) n;
        d_one : Equal Nat (positive_product_tail d Zero) d = (proof zero_l for add) d
      in
        trans
          Nat
          (mul (mul n (Suc Zero)) (Suc d))
          (mul n (Suc d))
          (mul n (Suc (positive_product_tail d Zero)))
          (cong Nat Nat (mul n (Suc Zero)) n (λv. mul v (Suc d)) n_one)
          (cong
            Nat
            Nat
            d
            (positive_product_tail d Zero)
            (λv. mul n (Suc v))
            (sym Nat (positive_product_tail d Zero) d d_one))
  }

pub proof zero_l for rational_mul
      (x : Rational)
    : rational_equiv (rational_mul rational_zero x) rational_zero =
  match x {
    MkRational n d ↦
      trans
        Nat
        (mul (mul Zero n) (Suc Zero))
        (mul Zero (Suc Zero))
        (mul Zero (denominator (rational_mul rational_zero (MkRational n d))))
        (cong Nat Nat (mul Zero n) Zero (λv. mul v (Suc Zero)) ((proof zero_l for mul) n))
        (sym
          Nat
          (mul Zero (denominator (rational_mul rational_zero (MkRational n d))))
          (mul Zero (Suc Zero))
          ((proof zero_l for mul) (denominator (rational_mul rational_zero (MkRational n d)))))
  }

pub proof distrib_l for rational_mul
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv
        (rational_mul x (rational_add y z))
        (rational_add (rational_mul x y) (rational_mul x z)) =
  match x {
    MkRational a bx ↦
      match y {
        MkRational b by ↦
          match z {
            MkRational c bz ↦ rational_distrib_cross a b c (Suc bx) (Suc by) (Suc bz)
          }
      }
  }

pub proof refl for rational_leq (x : Rational) : Equal Bool (rational_leq x x) True =
  (proof refl for leq_nat) (mul (numerator x) (denominator x))

pub proof trans for rational_leq
      (x : Rational) (y : Rational) (z : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq y z) True
      → Equal Bool (rational_leq x z) True =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦
          match z {
            MkRational e bz ↦ λxy. λyz. rational_leq_cross_trans a bx c by e bz xy yz
          }
      }
  }

pub proof antisym for rational_leq
      (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq y x) True
      → rational_equiv x y =
  λxy.
    λyx.
      (proof antisym for leq_nat)
        (mul (numerator x) (denominator y))
        (mul (numerator y) (denominator x))
        xy
        yx

pub proof total for rational_leq
      (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) False → Equal Bool (rational_leq y x) True =
  rational_leq_false_opposite
    (mul (numerator x) (denominator y))
    (mul (numerator y) (denominator x))

pub theorem rational_add_mono
      (x : Rational) (y : Rational) (z : Rational) (w : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq z w) True
      → Equal Bool (rational_leq (rational_add x z) (rational_add y w)) True =
  match x {
    MkRational a ax ↦
      match y {
        MkRational b bx ↦
          match z {
            MkRational c cx ↦
              match w {
                MkRational d dx ↦
                  λxy.
                    λzw.
                      rational_add_cross_mono a (Suc ax) b (Suc bx) c (Suc cx) d (Suc dx) xy zw
              }
          }
      }
  }

pub proof self_is_zero for rational_sub
      (x : Rational)
    : rational_equiv (rational_sub x x) rational_zero =
  match x {
    MkRational a b ↦
      trans
        Nat
        (mul (sub (mul a (Suc b)) (mul a (Suc b))) (Suc Zero))
        (mul Zero (Suc Zero))
        (mul Zero (Suc (positive_product_tail b b)))
        (cong
          Nat
          Nat
          (sub (mul a (Suc b)) (mul a (Suc b)))
          Zero
          (λv. mul v (Suc Zero))
          ((proof self_is_zero for sub) (mul a (Suc b))))
        (sym
          Nat
          (mul Zero (Suc (positive_product_tail b b)))
          Zero
          ((proof zero_l for mul) (Suc (positive_product_tail b b))))
  }

pub proof add_cancel for rational_sub
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq y x) → rational_equiv (rational_add (rational_sub x y) y) x =
  match x {
    MkRational a bx ↦
      match y {
        MkRational c by ↦ λordered. rational_sub_add_cancel_cross a c (Suc bx) (Suc by) ordered
      }
  }

pub proof add_distrib for rational_sub
      (x1 : Rational) (y1 : Rational) (x2 : Rational) (y2 : Rational)
    : IsTrue (rational_leq y1 x1)
      → IsTrue (rational_leq y2 x2)
      → rational_equiv
        (rational_sub (rational_add x1 x2) (rational_add y1 y2))
        (rational_add (rational_sub x1 y1) (rational_sub x2 y2)) =
  λfirst_ordered.
    λsecond_ordered.
      let
        high = rational_add x1 x2;
        low = rational_add y1 y2;
        total_width = rational_sub high low;
        first_width = rational_sub x1 y1;
        second_width = rational_sub x2 y2;
        separate_widths = rational_add first_width second_width;
        total_cancel : rational_equiv (rational_add total_width low) high =
          (proof add_cancel for rational_sub)
            high
            low
            (rational_add_mono y1 x1 y2 x2 first_ordered second_ordered);
        separate_cancel : rational_equiv (rational_add separate_widths low) high =
          (proof trans for rational_equiv)
            (rational_add separate_widths low)
            (rational_add (rational_add first_width y1) (rational_add second_width y2))
            high
            (rational_add_interchange first_width second_width y1 y2)
            ((proof respect for rational_add)
              (rational_add first_width y1)
              x1
              (rational_add second_width y2)
              x2
              ((proof add_cancel for rational_sub) x1 y1 first_ordered)
              ((proof add_cancel for rational_sub) x2 y2 second_ordered));
        same_after_add : rational_equiv
          (rational_add total_width low)
          (rational_add separate_widths low) =
          (proof trans for rational_equiv)
            (rational_add total_width low)
            high
            (rational_add separate_widths low)
            total_cancel
            ((proof sym for rational_equiv)
              (rational_add separate_widths low)
              high
              separate_cancel)
      in
        rational_add_cancel_r total_width separate_widths low same_after_add

pub theorem rational_sub_mono
      (x : Rational) (y : Rational) (z : Rational) (w : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq w z) True
      → Equal Bool (rational_leq (rational_sub x z) (rational_sub y w)) True =
  match x {
    MkRational a ax ↦
      match y {
        MkRational b bx ↦
          match z {
            MkRational c cx ↦
              match w {
                MkRational d dx ↦
                  λxy.
                    λwz.
                      rational_sub_cross_mono a (Suc ax) b (Suc bx) c (Suc cx) d (Suc dx) xy wz
              }
          }
      }
  }

pub theorem rational_mul_mono
      (x : Rational) (y : Rational) (z : Rational) (w : Rational)
    : Equal Bool (rational_leq x y) True
      → Equal Bool (rational_leq z w) True
      → Equal Bool (rational_leq (rational_mul x z) (rational_mul y w)) True =
  match x {
    MkRational a ax ↦
      match y {
        MkRational b bx ↦
          match z {
            MkRational c cx ↦
              match w {
                MkRational d dx ↦
                  λxy.
                    λzw.
                      rational_mul_cross_mono a (Suc ax) b (Suc bx) c (Suc cx) d (Suc dx) xy zw
              }
          }
      }
  }

pub fn rational_min (x : Rational) (y : Rational) : Rational =
  rational_pick (rational_leq x y) x y

pub fn rational_max (x : Rational) (y : Rational) : Rational =
  rational_pick (rational_leq x y) y x

pub proof leq_left for rational_min
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq (rational_min x y) x) =
  pick_leq_first (rational_leq x y) x y Refl

pub proof leq_right for rational_min
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq (rational_min x y) y) =
  pick_leq_second (rational_leq x y) x y Refl

pub proof greatest for rational_min
      (x : Rational) (y : Rational) (k : Rational)
    : IsTrue (rational_leq k x)
      → IsTrue (rational_leq k y)
      → IsTrue (rational_leq k (rational_min x y)) =
  pick_greatest (rational_leq x y) x y k

pub proof left_leq for rational_max
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq x (rational_max x y)) =
  pick_max_first (rational_leq x y) x y Refl

pub proof right_leq for rational_max
      (x : Rational) (y : Rational)
    : IsTrue (rational_leq y (rational_max x y)) =
  pick_max_second (rational_leq x y) x y Refl

pub proof least for rational_max
      (x : Rational) (y : Rational) (k : Rational)
    : IsTrue (rational_leq x k)
      → IsTrue (rational_leq y k)
      → IsTrue (rational_leq (rational_max x y) k) =
  λhx. λhy. pick_least (rational_leq x y) y x k hy hx

fn rational_pick (b : Bool) (x : Rational) (y : Rational) : Rational =
  match b {
    True ↦ x;
    False ↦ y
  }

theorem pick_leq_first
      (b : Bool) (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) b → IsTrue (rational_leq (rational_pick b x y) x) =
  match b {
    True ↦ λh. (proof refl for rational_leq) x;
    False ↦ λh. (proof total for rational_leq) x y h
  }

theorem pick_leq_second
      (b : Bool) (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) b → IsTrue (rational_leq (rational_pick b x y) y) =
  match b {
    True ↦ λh. h;
    False ↦ λh. (proof refl for rational_leq) y
  }

theorem pick_max_first
      (b : Bool) (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) b → IsTrue (rational_leq x (rational_pick b y x)) =
  match b {
    True ↦ λh. h;
    False ↦ λh. (proof refl for rational_leq) x
  }

theorem pick_max_second
      (b : Bool) (x : Rational) (y : Rational)
    : Equal Bool (rational_leq x y) b → IsTrue (rational_leq y (rational_pick b y x)) =
  match b {
    True ↦ λh. (proof refl for rational_leq) y;
    False ↦ λh. (proof total for rational_leq) x y h
  }

theorem pick_greatest
      (b : Bool) (x : Rational) (y : Rational) (k : Rational)
    : IsTrue (rational_leq k x)
      → IsTrue (rational_leq k y)
      → IsTrue (rational_leq k (rational_pick b x y)) =
  match b {
    True ↦ λhx. λhy. hx;
    False ↦ λhx. λhy. hy
  }

theorem pick_least
      (b : Bool) (x : Rational) (y : Rational) (k : Rational)
    : IsTrue (rational_leq x k)
      → IsTrue (rational_leq y k)
      → IsTrue (rational_leq (rational_pick b x y) k) =
  match b {
    True ↦ λhx. λhy. hx;
    False ↦ λhx. λhy. hy
  }
```

## 3. Using it

Different representations can denote the same value. A client constructs
`1/2` and `2/4` independently and compares them through `rational_equiv`:

```ken example
const half : Rational = MkRational (Suc Zero) (Suc Zero)

const two_quarters : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

theorem example_equal_values : rational_equiv half two_quarters = Proved
```

## 4. Laws & proofs

The proof families below use checked natural-number algebra from
`Data.Numeric.Nat.Arithmetic`; neither arithmetic nor order is redefined
inside this package. Positivity follows from the constructor's `Suc` field;
the local product-tail equation relates the resulting successor to the exact
product of two denominators. Both public denominator equations then reduce
against that construction. Transitivity reorders the two cross equations and
cancels the positive middle denominator.

Operand respect uses both equivalence hypotheses independently. Multiplication
shuffles the four natural factors until those hypotheses can replace the two
paired products. Addition first distributes each cross sum, then reuses the
same paired-product equalities. Boolean order respect compares equivalent
pairs after scaling by positive denominator products in both directions; the
checked Nat order laws carry each comparison into and back from those scales.
Order transitivity scales both adjacent comparisons by the opposite positive
denominator, joins them at the shared middle product, then cancels the common
positive factor. Antisymmetry follows on the two cross products, while Nat
totality and the actual false comparison force the reverse comparison.
Addition and multiplication preserve paired rational inequalities: the
monotonicity proofs distribute and permute natural cross products, lift both
input bounds through non-negative factors, and reassemble checked Nat order.

Min and max choose a stored operand after one rational comparison. Each
comparison branch discharges its own bounds from reflexivity, the checked
false-comparison converse, or the caller's hypotheses. A private Boolean pick
binds the computed comparison explicitly, so a dependent match refines that
exact occurrence. The six public attached proofs expose both operand bounds
and the greatest-lower/least-upper properties without adding assumptions.

Commutativity and multiplicative associativity lift the matching natural-number
algebra to both numerator and denominator. Additive associativity expands the
cross sums and re-associates three natural summands. Left distributivity
expands the numerator and refactors the two resulting products around their
shared denominator. Every final equation is value equivalence, not structural
identity of two carrier spellings.

Subtraction's self-zero proof reduces Nat subtraction of a cross product from
itself. Its ordered add-cancellation proof distributes the two numerator
products, uses Nat `sub::add_cancel`, then reorders their common denominator
factors. The two-input distribution proof adds the smaller values back to each
truncated difference, rearranges the rational sums, and cancels their common
right addend with private cross-product lemmas. Both order premises are needed;
no subtraction identity is asserted for arbitrary reversed operands.
`rational_sub_mono` cross-multiplies both differences, distributes
multiplication over truncated subtraction with `mul_sub_distrib_l`, and
applies `sub::left_mono` and `sub::right_antitone` to the rearranged
cross products without assuming either difference is nonnegative.

```ken
theorem positive_product_factor
      (bx : Nat) (by : Nat)
    : Equal Nat (Suc (positive_product_tail bx by)) (mul (Suc bx) (Suc by)) =
  Refl

theorem mul_twice_shift
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (mul (mul a b) b) c) (mul a (mul (mul c b) b)) =
  let
    initial = mul (mul (mul a b) b) c;
    moved = mul (mul (mul a b) c) b;
    paired = mul (mul (mul a c) b) b;
    grouped = mul (mul a c) (mul b b);
    nested = mul a (mul c (mul b b));
    final_value = mul a (mul (mul c b) b)
  in
    trans
      Nat
      initial
      moved
      final_value
      (mul_swap_tail (mul a b) b c)
      (trans
        Nat
        moved
        paired
        final_value
        (cong Nat Nat (mul (mul a b) c) (mul (mul a c) b) (λv. mul v b) (mul_swap_tail a b c))
        (trans
          Nat
          paired
          grouped
          final_value
          (sym Nat grouped paired ((proof assoc for mul) (mul a c) b b))
          (trans
            Nat
            grouped
            nested
            final_value
            (sym Nat nested grouped ((proof assoc for mul) a c (mul b b)))
            (cong
              Nat
              Nat
              (mul c (mul b b))
              (mul (mul c b) b)
              (mul a)
              ((proof assoc for mul) c b b)))))

theorem rational_sub_add_cancel_cross
      (a : Nat) (c : Nat) (aa : Nat) (cc : Nat)
    : IsTrue (leq_nat (mul c aa) (mul a cc))
      → Equal Nat
        (mul (add (mul (sub (mul a cc) (mul c aa)) cc) (mul c (mul aa cc))) aa)
        (mul a (mul (mul aa cc) cc)) =
  λordered.
    let
      difference = sub (mul a cc) (mul c aa);
      initial = mul (add (mul difference cc) (mul c (mul aa cc))) aa;
      associated = mul (add (mul difference cc) (mul (mul c aa) cc)) aa;
      distributed = mul (mul (add difference (mul c aa)) cc) aa;
      cancelled = mul (mul (mul a cc) cc) aa;
      final_value = mul a (mul (mul aa cc) cc)
    in
      trans
        Nat
        initial
        associated
        final_value
        (cong
          Nat
          Nat
          (mul c (mul aa cc))
          (mul (mul c aa) cc)
          (λv. mul (add (mul difference cc) v) aa)
          ((proof assoc for mul) c aa cc))
        (trans
          Nat
          associated
          distributed
          final_value
          (cong
            Nat
            Nat
            (add (mul difference cc) (mul (mul c aa) cc))
            (mul (add difference (mul c aa)) cc)
            (λv. mul v aa)
            (sym
              Nat
              (mul (add difference (mul c aa)) cc)
              (add (mul difference cc) (mul (mul c aa) cc))
              (mul_add_distrib_l difference (mul c aa) cc)))
          (trans
            Nat
            distributed
            cancelled
            final_value
            (cong
              Nat
              Nat
              (add difference (mul c aa))
              (mul a cc)
              (λv. mul (mul v cc) aa)
              ((proof add_cancel for sub) (mul c aa) (mul a cc) ordered))
            (mul_twice_shift a cc aa)))

theorem rational_add_cancel_cross
      (a : Nat) (b : Nat) (c : Nat) (aa : Nat) (bb : Nat) (tail : Nat)
    : Equal Nat
        (mul (add (mul a (Suc tail)) (mul c aa)) (mul bb (Suc tail)))
        (mul (add (mul b (Suc tail)) (mul c bb)) (mul aa (Suc tail)))
      → Equal Nat (mul a bb) (mul b aa) =
  λsame_sum.
    let
      cc = Suc tail;
      first = mul (mul a cc) (mul bb cc);
      shared_left = mul (mul c aa) (mul bb cc);
      second = mul (mul b cc) (mul aa cc);
      shared_right = mul (mul c bb) (mul aa cc);
      first_scaled = mul (mul a bb) (mul cc cc);
      second_scaled = mul (mul b aa) (mul cc cc);
      initial_left = mul (add (mul a cc) (mul c aa)) (mul bb cc);
      initial_right = mul (add (mul b cc) (mul c bb)) (mul aa cc);
      expanded_left = add first shared_left;
      expanded_right = add second shared_right;
      aligned_left = add first_scaled shared_right;
      aligned_right = add second_scaled shared_right;
      left_expansion : Equal Nat initial_left expanded_left =
        mul_add_distrib_l (mul a cc) (mul c aa) (mul bb cc);
      right_expansion : Equal Nat initial_right expanded_right =
        mul_add_distrib_l (mul b cc) (mul c bb) (mul aa cc);
      expanded_equality : Equal Nat expanded_left expanded_right =
        trans
          Nat
          expanded_left
          initial_left
          expanded_right
          (sym Nat initial_left expanded_left left_expansion)
          (trans Nat initial_left initial_right expanded_right same_sum right_expansion);
      left_alignment : Equal Nat expanded_left aligned_left =
        add_cong2
          first
          first_scaled
          shared_left
          shared_right
          (mul_shuffle a cc bb cc)
          (mul_shuffle c aa bb cc);
      right_alignment : Equal Nat expanded_right aligned_right =
        add_cong2 second second_scaled shared_right shared_right (mul_shuffle b cc aa cc) Refl;
      aligned_equality : Equal Nat aligned_left aligned_right =
        trans
          Nat
          aligned_left
          expanded_left
          aligned_right
          (sym Nat expanded_left aligned_left left_alignment)
          (trans
            Nat
            expanded_left
            expanded_right
            aligned_right
            expanded_equality
            right_alignment);
      scaled_equality : Equal Nat first_scaled second_scaled =
        add_cancel_r first_scaled second_scaled shared_right aligned_equality
    in
      (proof cancel_r for mul)
        (mul a bb)
        (mul b aa)
        (positive_product_tail tail tail)
        scaled_equality

theorem rational_add_cancel_r
      (x : Rational) (y : Rational) (z : Rational)
    : rational_equiv (rational_add x z) (rational_add y z) → rational_equiv x y =
  match x {
    MkRational a bx ↦
      match y {
        MkRational b by ↦
          match z {
            MkRational c bz ↦
              λsame_sum. rational_add_cancel_cross a b c (Suc bx) (Suc by) bz same_sum
          }
      }
  }

theorem rational_add_interchange
      (a : Rational) (b : Rational) (c : Rational) (d : Rational)
    : rational_equiv
        (rational_add (rational_add a b) (rational_add c d))
        (rational_add (rational_add a c) (rational_add b d)) =
  let
    initial = rational_add (rational_add a b) (rational_add c d);
    right_nested = rational_add a (rational_add b (rational_add c d));
    middle_regrouped = rational_add a (rational_add (rational_add b c) d);
    middle_swapped = rational_add a (rational_add (rational_add c b) d);
    left_nested = rational_add a (rational_add c (rational_add b d));
    final_sum = rational_add (rational_add a c) (rational_add b d)
  in
    (proof trans for rational_equiv)
      initial
      right_nested
      final_sum
      ((proof sym for rational_equiv)
        right_nested
        initial
        ((proof assoc for rational_add) a b (rational_add c d)))
      ((proof trans for rational_equiv)
        right_nested
        middle_regrouped
        final_sum
        ((proof respect for rational_add)
          a
          a
          (rational_add b (rational_add c d))
          (rational_add (rational_add b c) d)
          ((proof refl for rational_equiv) a)
          ((proof assoc for rational_add) b c d))
        ((proof trans for rational_equiv)
          middle_regrouped
          middle_swapped
          final_sum
          ((proof respect for rational_add)
            a
            a
            (rational_add (rational_add b c) d)
            (rational_add (rational_add c b) d)
            ((proof refl for rational_equiv) a)
            ((proof respect for rational_add)
              (rational_add b c)
              (rational_add c b)
              d
              d
              ((proof comm for rational_add) b c)
              ((proof refl for rational_equiv) d)))
          ((proof trans for rational_equiv)
            middle_swapped
            left_nested
            final_sum
            ((proof respect for rational_add)
              a
              a
              (rational_add (rational_add c b) d)
              (rational_add c (rational_add b d))
              ((proof refl for rational_equiv) a)
              ((proof sym for rational_equiv)
                (rational_add c (rational_add b d))
                (rational_add (rational_add c b) d)
                ((proof assoc for rational_add) c b d)))
            ((proof assoc for rational_add) a c (rational_add b d)))))

theorem add_cross_respect
      (a : Nat)
      (b : Nat)
      (c : Nat)
      (d : Nat)
      (ap : Nat)
      (bp : Nat)
      (cp : Nat)
      (dp : Nat)
      (ha : Equal Nat (mul a bp) (mul ap b))
      (hc : Equal Nat (mul c dp) (mul cp d))
    : Equal Nat
        (mul (add (mul a d) (mul c b)) (mul bp dp))
        (mul (add (mul ap dp) (mul cp bp)) (mul b d)) =
  let
    left_cross_product : Equal Nat (mul (mul a d) (mul bp dp)) (mul (mul ap dp) (mul b d)) =
      cross_first a b d ap bp dp ha;
    right_cross_product : Equal Nat (mul (mul c b) (mul bp dp)) (mul (mul cp bp) (mul b d)) =
      cross_second b c d bp cp dp hc
  in
    trans
      Nat
      (mul (add (mul a d) (mul c b)) (mul bp dp))
      (add (mul (mul a d) (mul bp dp)) (mul (mul c b) (mul bp dp)))
      (mul (add (mul ap dp) (mul cp bp)) (mul b d))
      (mul_add_distrib_l (mul a d) (mul c b) (mul bp dp))
      (trans
        Nat
        (add (mul (mul a d) (mul bp dp)) (mul (mul c b) (mul bp dp)))
        (add (mul (mul ap dp) (mul b d)) (mul (mul cp bp) (mul b d)))
        (mul (add (mul ap dp) (mul cp bp)) (mul b d))
        (trans
          Nat
          (add (mul (mul a d) (mul bp dp)) (mul (mul c b) (mul bp dp)))
          (add (mul (mul ap dp) (mul b d)) (mul (mul c b) (mul bp dp)))
          (add (mul (mul ap dp) (mul b d)) (mul (mul cp bp) (mul b d)))
          (cong
            Nat
            Nat
            (mul (mul a d) (mul bp dp))
            (mul (mul ap dp) (mul b d))
            (λv. add v (mul (mul c b) (mul bp dp)))
            left_cross_product)
          (cong
            Nat
            Nat
            (mul (mul c b) (mul bp dp))
            (mul (mul cp bp) (mul b d))
            (λv. add (mul (mul ap dp) (mul b d)) v)
            right_cross_product))
        (sym
          Nat
          (mul (add (mul ap dp) (mul cp bp)) (mul b d))
          (add (mul (mul ap dp) (mul b d)) (mul (mul cp bp) (mul b d)))
          (mul_add_distrib_l (mul ap dp) (mul cp bp) (mul b d))))

theorem mul_cross_respect
      (a : Nat)
      (b : Nat)
      (c : Nat)
      (d : Nat)
      (ap : Nat)
      (bp : Nat)
      (cp : Nat)
      (dp : Nat)
      (ha : Equal Nat (mul a bp) (mul ap b))
      (hc : Equal Nat (mul c dp) (mul cp d))
    : Equal Nat (mul (mul a c) (mul bp dp)) (mul (mul ap cp) (mul b d)) =
  trans
    Nat
    (mul (mul a c) (mul bp dp))
    (mul (mul a bp) (mul c dp))
    (mul (mul ap cp) (mul b d))
    (mul_shuffle a c bp dp)
    (trans
      Nat
      (mul (mul a bp) (mul c dp))
      (mul (mul ap b) (mul cp d))
      (mul (mul ap cp) (mul b d))
      (mul_cong2 (mul a bp) (mul ap b) (mul c dp) (mul cp d) ha hc)
      (mul_shuffle ap b cp d))

theorem leq_cross_respect
      (a : Nat)
      (bx : Nat)
      (c : Nat)
      (by : Nat)
      (ap : Nat)
      (bxp : Nat)
      (cp : Nat)
      (byp : Nat)
      (ha : Equal Nat (mul a (Suc bxp)) (mul ap (Suc bx)))
      (hc : Equal Nat (mul c (Suc byp)) (mul cp (Suc by)))
    : Equal Bool
        (leq_nat (mul a (Suc by)) (mul c (Suc bx)))
        (leq_nat (mul ap (Suc byp)) (mul cp (Suc bxp))) =
  let
    left = mul a (Suc by);
    right = mul c (Suc bx);
    next_left = mul ap (Suc byp);
    next_right = mul cp (Suc bxp);
    before_factor = mul (Suc bxp) (Suc byp);
    after_factor = mul (Suc bx) (Suc by);
    cross_left : Equal Nat (mul left before_factor) (mul next_left after_factor) =
      cross_first a (Suc bx) (Suc by) ap (Suc bxp) (Suc byp) ha;
    cross_right : Equal Nat (mul right before_factor) (mul next_right after_factor) =
      cross_second (Suc bx) c (Suc by) (Suc bxp) cp (Suc byp) hc;
    comparison : Equal Bool
      (leq_nat (mul left before_factor) (mul right before_factor))
      (leq_nat (mul next_left after_factor) (mul next_right after_factor)) =
      leq_cong_two
        (mul left before_factor)
        (mul next_left after_factor)
        (mul right before_factor)
        (mul next_right after_factor)
        cross_left
        cross_right;
    forward : Equal Bool
      (leq_nat left right)
      True → Equal Bool
      (leq_nat next_left next_right)
      True =
      λh.
        leq_nat_mul_right_cancel
          next_left
          next_right
          (positive_product_tail bx by)
          (trans
            Bool
            (leq_nat (mul next_left after_factor) (mul next_right after_factor))
            (leq_nat (mul left before_factor) (mul right before_factor))
            True
            (sym
              Bool
              (leq_nat (mul left before_factor) (mul right before_factor))
              (leq_nat (mul next_left after_factor) (mul next_right after_factor))
              comparison)
            (leq_nat_mul_right_mono left right (positive_product_tail bxp byp) h));
    backward : Equal Bool
      (leq_nat next_left next_right)
      True → Equal Bool
      (leq_nat left right)
      True =
      λh.
        leq_nat_mul_right_cancel
          left
          right
          (positive_product_tail bxp byp)
          (trans
            Bool
            (leq_nat (mul left before_factor) (mul right before_factor))
            (leq_nat (mul next_left after_factor) (mul next_right after_factor))
            True
            comparison
            (leq_nat_mul_right_mono next_left next_right (positive_product_tail bx by) h))
  in
    bool_eq_from_iff (leq_nat left right) (leq_nat next_left next_right) forward backward

theorem rational_leq_cross_trans
      (a : Nat)
      (bx : Nat)
      (c : Nat)
      (by : Nat)
      (e : Nat)
      (bz : Nat)
      (xy : Equal Bool (leq_nat (mul a (Suc by)) (mul c (Suc bx))) True)
      (yz : Equal Bool (leq_nat (mul c (Suc bz)) (mul e (Suc by))) True)
    : Equal Bool (leq_nat (mul a (Suc bz)) (mul e (Suc bx))) True =
  let
    first = mul a (Suc by);
    middle_x = mul c (Suc bx);
    middle_z = mul c (Suc bz);
    last = mul e (Suc by);
    first_scaled : Equal Bool (leq_nat (mul first (Suc bz)) (mul middle_x (Suc bz))) True =
      leq_nat_mul_right_mono first middle_x bz xy;
    second_scaled : Equal Bool (leq_nat (mul middle_z (Suc bx)) (mul last (Suc bx))) True =
      leq_nat_mul_right_mono middle_z last bx yz;
    joined_middle : Equal Bool (leq_nat (mul middle_x (Suc bz)) (mul last (Suc bx))) True =
      trans
        Bool
        (leq_nat (mul middle_x (Suc bz)) (mul last (Suc bx)))
        (leq_nat (mul middle_z (Suc bx)) (mul last (Suc bx)))
        True
        (leq_cong_two
          (mul middle_x (Suc bz))
          (mul middle_z (Suc bx))
          (mul last (Suc bx))
          (mul last (Suc bx))
          (mul_swap_tail c (Suc bx) (Suc bz))
          Refl)
        second_scaled;
    scaled : Equal Bool (leq_nat (mul first (Suc bz)) (mul last (Suc bx))) True =
      (proof trans for leq_nat)
        (mul first (Suc bz))
        (mul middle_x (Suc bz))
        (mul last (Suc bx))
        first_scaled
        joined_middle;
    reordered : Equal Bool
      (leq_nat (mul (mul a (Suc bz)) (Suc by)) (mul (mul e (Suc bx)) (Suc by)))
      True =
      trans
        Bool
        (leq_nat (mul (mul a (Suc bz)) (Suc by)) (mul (mul e (Suc bx)) (Suc by)))
        (leq_nat (mul first (Suc bz)) (mul last (Suc bx)))
        True
        (leq_cong_two
          (mul (mul a (Suc bz)) (Suc by))
          (mul first (Suc bz))
          (mul (mul e (Suc bx)) (Suc by))
          (mul last (Suc bx))
          (sym
            Nat
            (mul first (Suc bz))
            (mul (mul a (Suc bz)) (Suc by))
            (mul_swap_tail a (Suc by) (Suc bz)))
          (sym
            Nat
            (mul last (Suc bx))
            (mul (mul e (Suc bx)) (Suc by))
            (mul_swap_tail e (Suc by) (Suc bx))))
        scaled
  in
    leq_nat_mul_right_cancel (mul a (Suc bz)) (mul e (Suc bx)) by reordered

theorem rational_leq_false_opposite
      (a : Nat) (b : Nat) (ab_false : Equal Bool (leq_nat a b) False)
    : Equal Bool (leq_nat b a) True =
  (proof left_false_elim for bool_or)
    (leq_nat a b)
    (leq_nat b a)
    ab_false
    ((proof total for leq_nat) a b)

fn positive_product_tail (a : Nat) (b : Nat) : Nat = add (mul (Suc a) b) a

theorem cross_first
      (a : Nat)
      (b : Nat)
      (d : Nat)
      (ap : Nat)
      (bp : Nat)
      (dp : Nat)
      (ha : Equal Nat (mul a bp) (mul ap b))
    : Equal Nat (mul (mul a d) (mul bp dp)) (mul (mul ap dp) (mul b d)) =
  trans
    Nat
    (mul (mul a d) (mul bp dp))
    (mul (mul a bp) (mul d dp))
    (mul (mul ap dp) (mul b d))
    (mul_shuffle a d bp dp)
    (trans
      Nat
      (mul (mul a bp) (mul d dp))
      (mul (mul ap b) (mul dp d))
      (mul (mul ap dp) (mul b d))
      (mul_cong2 (mul a bp) (mul ap b) (mul d dp) (mul dp d) ha ((proof comm for mul) d dp))
      (sym Nat (mul (mul ap dp) (mul b d)) (mul (mul ap b) (mul dp d)) (mul_shuffle ap dp b d)))

theorem cross_second
      (b : Nat)
      (c : Nat)
      (d : Nat)
      (bp : Nat)
      (cp : Nat)
      (dp : Nat)
      (hc : Equal Nat (mul c dp) (mul cp d))
    : Equal Nat (mul (mul c b) (mul bp dp)) (mul (mul cp bp) (mul b d)) =
  trans
    Nat
    (mul (mul c b) (mul bp dp))
    (mul (mul c dp) (mul b bp))
    (mul (mul cp bp) (mul b d))
    (trans
      Nat
      (mul (mul c b) (mul bp dp))
      (mul (mul c b) (mul dp bp))
      (mul (mul c dp) (mul b bp))
      (cong Nat Nat (mul bp dp) (mul dp bp) (λv. mul (mul c b) v) ((proof comm for mul) bp dp))
      (mul_shuffle c b dp bp))
    (trans
      Nat
      (mul (mul c dp) (mul b bp))
      (mul (mul cp d) (mul bp b))
      (mul (mul cp bp) (mul b d))
      (mul_cong2 (mul c dp) (mul cp d) (mul b bp) (mul bp b) hc ((proof comm for mul) b bp))
      (trans
        Nat
        (mul (mul cp d) (mul bp b))
        (mul (mul cp bp) (mul d b))
        (mul (mul cp bp) (mul b d))
        (sym
          Nat
          (mul (mul cp bp) (mul d b))
          (mul (mul cp d) (mul bp b))
          (mul_shuffle cp bp d b))
        (cong Nat Nat (mul d b) (mul b d) (λv. mul (mul cp bp) v) ((proof comm for mul) d b))))

theorem mul_cong2
      (a : Nat) (b : Nat) (c : Nat) (d : Nat) (hab : Equal Nat a b) (hcd : Equal Nat c d)
    : Equal Nat (mul a c) (mul b d) =
  trans
    Nat
    (mul a c)
    (mul b c)
    (mul b d)
    (cong Nat Nat a b (λv. mul v c) hab)
    (cong Nat Nat c d (mul b) hcd)

theorem leq_cong_two
      (a : Nat) (ap : Nat) (b : Nat) (bp : Nat) (ha : Equal Nat a ap) (hb : Equal Nat b bp)
    : Equal Bool (leq_nat a b) (leq_nat ap bp) =
  trans
    Bool
    (leq_nat a b)
    (leq_nat ap b)
    (leq_nat ap bp)
    (cong Nat Bool a ap (λv. leq_nat v b) ha)
    (cong Nat Bool b bp (leq_nat ap) hb)

theorem bool_eq_from_iff
      (p : Bool)
    : (q : Bool)
      → (Equal Bool p True → Equal Bool q True)
      → (Equal Bool q True → Equal Bool p True)
      → Equal Bool p q =
  match p {
    True ↦
      λq.
        match q {
          True ↦ λforward. λbackward. Proved;
          False ↦ λforward. λbackward. absurd (forward Proved)
        };
    False ↦
      λq.
        match q {
          True ↦ λforward. λbackward. absurd (backward Proved);
          False ↦ λforward. λbackward. Proved
        }
  }

theorem add_assoc_cross
      (a : Nat) (b : Nat) (c : Nat) (d : Nat) (e : Nat) (f : Nat)
    : Equal Nat
        (add (mul a (mul e f)) (mul (add (mul b f) (mul c e)) d))
        (add (mul (add (mul a e) (mul b d)) f) (mul c (mul d e))) =
  let
    first = mul a (mul e f);
    expanded_b = mul (mul b f) d;
    expanded_c = mul (mul c e) d;
    middle = mul (mul b d) f;
    last = mul c (mul d e);
    first_associated = mul (mul a e) f;
    initial = add first (mul (add (mul b f) (mul c e)) d);
    distributed = add first (add expanded_b expanded_c);
    rearranged = add first (add middle last);
    first_reassociated = add first_associated (add middle last);
    sum_reassociated = add (add first_associated middle) last;
    final_sum = add (mul (add (mul a e) (mul b d)) f) last
  in
    trans
      Nat
      initial
      distributed
      final_sum
      (cong
        Nat
        Nat
        (mul (add (mul b f) (mul c e)) d)
        (add expanded_b expanded_c)
        (add first)
        (mul_add_distrib_l (mul b f) (mul c e) d))
      (trans
        Nat
        distributed
        rearranged
        final_sum
        (cong
          Nat
          Nat
          (add expanded_b expanded_c)
          (add middle last)
          (add first)
          (add_cong2
            expanded_b
            middle
            expanded_c
            last
            (mul_swap_tail b f d)
            (mul_reorder_right c e d)))
        (trans
          Nat
          rearranged
          first_reassociated
          final_sum
          (cong
            Nat
            Nat
            first
            first_associated
            (λv. add v (add middle last))
            ((proof assoc for mul) a e f))
          (trans
            Nat
            first_reassociated
            sum_reassociated
            final_sum
            ((proof assoc for add) first_associated middle last)
            (cong
              Nat
              Nat
              (add first_associated middle)
              (mul (add (mul a e) (mul b d)) f)
              (λv. add v last)
              (sym
                Nat
                (mul (add (mul a e) (mul b d)) f)
                (add first_associated middle)
                (mul_add_distrib_l (mul a e) (mul b d) f))))))

theorem mul_reorder_right
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (mul a b) c) (mul a (mul c b)) =
  trans
    Nat
    (mul (mul a b) c)
    (mul a (mul b c))
    (mul a (mul c b))
    (sym Nat (mul a (mul b c)) (mul (mul a b) c) ((proof assoc for mul) a b c))
    (cong Nat Nat (mul b c) (mul c b) (mul a) ((proof comm for mul) b c))

theorem rational_distrib_cross
      (a : Nat) (b : Nat) (c : Nat) (d : Nat) (e : Nat) (f : Nat)
    : Equal Nat
        (mul (mul a (add (mul b f) (mul c e))) (mul (mul d e) (mul d f)))
        (mul (add (mul (mul a b) (mul d f)) (mul (mul a c) (mul d e))) (mul d (mul e f))) =
  let
    left_numerator = mul a (add (mul b f) (mul c e));
    expanded_left = mul a (mul b f);
    expanded_right = mul a (mul c e);
    right_left = mul (mul a b) (mul d f);
    right_right = mul (mul a c) (mul d e);
    repeated_denominator = mul (mul d e) (mul d f);
    common_denominator = mul d (mul e f);
    initial = mul left_numerator repeated_denominator;
    expanded = mul (add expanded_left expanded_right) repeated_denominator;
    distributed =
      add (mul expanded_left repeated_denominator) (mul expanded_right repeated_denominator);
    rearranged = add (mul right_left common_denominator) (mul right_right common_denominator);
    final_product = mul (add right_left right_right) common_denominator
  in
    trans
      Nat
      initial
      expanded
      final_product
      (cong
        Nat
        Nat
        left_numerator
        (add expanded_left expanded_right)
        (λv. mul v repeated_denominator)
        (mul_add_distrib_r a (mul b f) (mul c e)))
      (trans
        Nat
        expanded
        distributed
        final_product
        (mul_add_distrib_l expanded_left expanded_right repeated_denominator)
        (trans
          Nat
          distributed
          rearranged
          final_product
          (add_cong2
            (mul expanded_left repeated_denominator)
            (mul right_left common_denominator)
            (mul expanded_right repeated_denominator)
            (mul right_right common_denominator)
            (distrib_cross_term a b f d e f)
            (distrib_cross_term a c e d e f))
          (sym
            Nat
            final_product
            rearranged
            (mul_add_distrib_l right_left right_right common_denominator))))

theorem add_cong2
      (a : Nat) (ap : Nat) (b : Nat) (bp : Nat) (ha : Equal Nat a ap) (hb : Equal Nat b bp)
    : Equal Nat (add a b) (add ap bp) =
  trans
    Nat
    (add a b)
    (add ap b)
    (add ap bp)
    (cong Nat Nat a ap (λv. add v b) ha)
    (cong Nat Nat b bp (add ap) hb)

theorem distrib_cross_term
      (a : Nat) (b : Nat) (c : Nat) (d : Nat) (e : Nat) (f : Nat)
    : Equal Nat
        (mul (mul a (mul b c)) (mul (mul d e) (mul d f)))
        (mul (mul (mul a b) (mul d c)) (mul d (mul e f))) =
  let
    numerator_factor = mul a (mul b c);
    associated_numerator = mul (mul a b) c;
    repeated_denominator = mul (mul d e) (mul d f);
    common_denominator = mul d (mul e f);
    left = mul numerator_factor repeated_denominator;
    reassociated = mul associated_numerator repeated_denominator;
    shared = mul associated_numerator (mul d common_denominator);
    right = mul (mul (mul a b) (mul d c)) common_denominator
  in
    trans
      Nat
      left
      reassociated
      right
      (cong
        Nat
        Nat
        numerator_factor
        associated_numerator
        (λv. mul v repeated_denominator)
        ((proof assoc for mul) a b c))
      (trans
        Nat
        reassociated
        shared
        right
        (cong
          Nat
          Nat
          repeated_denominator
          (mul d common_denominator)
          (λv. mul associated_numerator v)
          (denominator_repeat d e f))
        (mul_factor_shuffle (mul a b) c d common_denominator))

theorem denominator_repeat
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (mul a b) (mul a c)) (mul a (mul a (mul b c))) =
  trans
    Nat
    (mul (mul a b) (mul a c))
    (mul (mul a a) (mul b c))
    (mul a (mul a (mul b c)))
    (mul_shuffle a b a c)
    (sym
      Nat
      (mul a (mul a (mul b c)))
      (mul (mul a a) (mul b c))
      ((proof assoc for mul) a a (mul b c)))

theorem mul_factor_shuffle
      (x : Nat) (c : Nat) (d : Nat) (y : Nat)
    : Equal Nat (mul (mul x c) (mul d y)) (mul (mul x (mul d c)) y) =
  trans
    Nat
    (mul (mul x c) (mul d y))
    (mul (mul x d) (mul c y))
    (mul (mul x (mul d c)) y)
    (mul_shuffle x c d y)
    (trans
      Nat
      (mul (mul x d) (mul c y))
      (mul (mul (mul x d) c) y)
      (mul (mul x (mul d c)) y)
      ((proof assoc for mul) (mul x d) c y)
      (cong
        Nat
        Nat
        (mul (mul x d) c)
        (mul x (mul d c))
        (λv. mul v y)
        (sym Nat (mul x (mul d c)) (mul (mul x d) c) ((proof assoc for mul) x d c))))

theorem mul_shuffle
      (a : Nat) (b : Nat) (c : Nat) (d : Nat)
    : Equal Nat (mul (mul a b) (mul c d)) (mul (mul a c) (mul b d)) =
  trans
    Nat
    (mul (mul a b) (mul c d))
    (mul (mul (mul a b) c) d)
    (mul (mul a c) (mul b d))
    ((proof assoc for mul) (mul a b) c d)
    (trans
      Nat
      (mul (mul (mul a b) c) d)
      (mul (mul (mul a c) b) d)
      (mul (mul a c) (mul b d))
      (cong Nat Nat (mul (mul a b) c) (mul (mul a c) b) (λv. mul v d) (mul_swap_tail a b c))
      (sym
        Nat
        (mul (mul a c) (mul b d))
        (mul (mul (mul a c) b) d)
        ((proof assoc for mul) (mul a c) b d)))

theorem mul_swap_tail
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (mul a b) c) (mul (mul a c) b) =
  trans
    Nat
    (mul (mul a b) c)
    (mul a (mul b c))
    (mul (mul a c) b)
    (sym Nat (mul a (mul b c)) (mul (mul a b) c) ((proof assoc for mul) a b c))
    (trans
      Nat
      (mul a (mul b c))
      (mul a (mul c b))
      (mul (mul a c) b)
      (cong Nat Nat (mul b c) (mul c b) (λv. mul a v) ((proof comm for mul) b c))
      ((proof assoc for mul) a c b))

theorem rational_add_cross_mono
      (a : Nat)
      (aa : Nat)
      (b : Nat)
      (bb : Nat)
      (c : Nat)
      (cc : Nat)
      (d : Nat)
      (dd : Nat)
      (xy : Equal Bool (leq_nat (mul a bb) (mul b aa)) True)
      (zw : Equal Bool (leq_nat (mul c dd) (mul d cc)) True)
    : Equal Bool
        (leq_nat
          (mul (add (mul a cc) (mul c aa)) (mul bb dd))
          (mul (add (mul b dd) (mul d bb)) (mul aa cc)))
        True =
  leq_rewrite
    (mul (add (mul a cc) (mul c aa)) (mul bb dd))
    (add (mul (mul a cc) (mul bb dd)) (mul (mul c aa) (mul bb dd)))
    (mul (add (mul b dd) (mul d bb)) (mul aa cc))
    (add (mul (mul b dd) (mul aa cc)) (mul (mul d bb) (mul aa cc)))
    (mul_add_distrib_l (mul a cc) (mul c aa) (mul bb dd))
    (mul_add_distrib_l (mul b dd) (mul d bb) (mul aa cc))
    (leq_nat_add_mono
      (mul (mul a cc) (mul bb dd))
      (mul (mul b dd) (mul aa cc))
      (mul (mul c aa) (mul bb dd))
      (mul (mul d bb) (mul aa cc))
      (leq_rewrite
        (mul (mul a cc) (mul bb dd))
        (mul (mul a bb) (mul cc dd))
        (mul (mul b dd) (mul aa cc))
        (mul (mul b aa) (mul cc dd))
        (mul_shuffle a cc bb dd)
        (trans
          Nat
          (mul (mul b dd) (mul aa cc))
          (mul (mul b aa) (mul dd cc))
          (mul (mul b aa) (mul cc dd))
          (mul_shuffle b dd aa cc)
          (mul_inner_comm (mul b aa) dd cc))
        (leq_nat_mul_right_mono_any (mul a bb) (mul b aa) (mul cc dd) xy))
      (leq_rewrite
        (mul (mul c aa) (mul bb dd))
        (mul (mul c dd) (mul aa bb))
        (mul (mul d bb) (mul aa cc))
        (mul (mul d cc) (mul aa bb))
        (trans
          Nat
          (mul (mul c aa) (mul bb dd))
          (mul (mul c aa) (mul dd bb))
          (mul (mul c dd) (mul aa bb))
          (mul_inner_comm (mul c aa) bb dd)
          (mul_shuffle c aa dd bb))
        (trans
          Nat
          (mul (mul d bb) (mul aa cc))
          (mul (mul d bb) (mul cc aa))
          (mul (mul d cc) (mul aa bb))
          (mul_inner_comm (mul d bb) aa cc)
          (trans
            Nat
            (mul (mul d bb) (mul cc aa))
            (mul (mul d cc) (mul bb aa))
            (mul (mul d cc) (mul aa bb))
            (mul_shuffle d bb cc aa)
            (mul_inner_comm (mul d cc) bb aa)))
        (leq_nat_mul_right_mono_any (mul c dd) (mul d cc) (mul aa bb) zw)))

theorem rational_mul_cross_mono
      (a : Nat)
      (aa : Nat)
      (b : Nat)
      (bb : Nat)
      (c : Nat)
      (cc : Nat)
      (d : Nat)
      (dd : Nat)
      (xy : Equal Bool (leq_nat (mul a bb) (mul b aa)) True)
      (zw : Equal Bool (leq_nat (mul c dd) (mul d cc)) True)
    : Equal Bool (leq_nat (mul (mul a c) (mul bb dd)) (mul (mul b d) (mul aa cc))) True =
  leq_rewrite
    (mul (mul a c) (mul bb dd))
    (mul (mul a bb) (mul c dd))
    (mul (mul b d) (mul aa cc))
    (mul (mul b aa) (mul d cc))
    (mul_shuffle a c bb dd)
    (mul_shuffle b d aa cc)
    ((proof trans for leq_nat)
      (mul (mul a bb) (mul c dd))
      (mul (mul b aa) (mul c dd))
      (mul (mul b aa) (mul d cc))
      (leq_nat_mul_right_mono_any (mul a bb) (mul b aa) (mul c dd) xy)
      (leq_nat_mul_left_mono_any (mul b aa) (mul c dd) (mul d cc) zw))

theorem rational_sub_cross_mono
      (a : Nat)
      (aa : Nat)
      (b : Nat)
      (bb : Nat)
      (c : Nat)
      (cc : Nat)
      (d : Nat)
      (dd : Nat)
      (xy : Equal Bool (leq_nat (mul a bb) (mul b aa)) True)
      (wz : Equal Bool (leq_nat (mul d cc) (mul c dd)) True)
    : Equal Bool
        (leq_nat
          (mul (sub (mul a cc) (mul c aa)) (mul bb dd))
          (mul (sub (mul b dd) (mul d bb)) (mul aa cc)))
        True =
  leq_rewrite
    (mul (sub (mul a cc) (mul c aa)) (mul bb dd))
    (sub (mul (mul a cc) (mul bb dd)) (mul (mul c aa) (mul bb dd)))
    (mul (sub (mul b dd) (mul d bb)) (mul aa cc))
    (sub (mul (mul b dd) (mul aa cc)) (mul (mul d bb) (mul aa cc)))
    (mul_sub_distrib_l (mul a cc) (mul c aa) (mul bb dd))
    (mul_sub_distrib_l (mul b dd) (mul d bb) (mul aa cc))
    ((proof trans for leq_nat)
      (sub (mul (mul a cc) (mul bb dd)) (mul (mul c aa) (mul bb dd)))
      (sub (mul (mul b dd) (mul aa cc)) (mul (mul c aa) (mul bb dd)))
      (sub (mul (mul b dd) (mul aa cc)) (mul (mul d bb) (mul aa cc)))
      ((proof left_mono for sub)
        (mul (mul a cc) (mul bb dd))
        (mul (mul b dd) (mul aa cc))
        (mul (mul c aa) (mul bb dd))
        (leq_rewrite
          (mul (mul a cc) (mul bb dd))
          (mul (mul a bb) (mul cc dd))
          (mul (mul b dd) (mul aa cc))
          (mul (mul b aa) (mul cc dd))
          (mul_shuffle a cc bb dd)
          (trans
            Nat
            (mul (mul b dd) (mul aa cc))
            (mul (mul b aa) (mul dd cc))
            (mul (mul b aa) (mul cc dd))
            (mul_shuffle b dd aa cc)
            (mul_inner_comm (mul b aa) dd cc))
          (leq_nat_mul_right_mono_any (mul a bb) (mul b aa) (mul cc dd) xy)))
      ((proof right_antitone for sub)
        (mul (mul b dd) (mul aa cc))
        (mul (mul d bb) (mul aa cc))
        (mul (mul c aa) (mul bb dd))
        (leq_rewrite
          (mul (mul d bb) (mul aa cc))
          (mul (mul d cc) (mul aa bb))
          (mul (mul c aa) (mul bb dd))
          (mul (mul c dd) (mul aa bb))
          (trans
            Nat
            (mul (mul d bb) (mul aa cc))
            (mul (mul d bb) (mul cc aa))
            (mul (mul d cc) (mul aa bb))
            (mul_inner_comm (mul d bb) aa cc)
            (trans
              Nat
              (mul (mul d bb) (mul cc aa))
              (mul (mul d cc) (mul bb aa))
              (mul (mul d cc) (mul aa bb))
              (mul_shuffle d bb cc aa)
              (mul_inner_comm (mul d cc) bb aa)))
          (trans
            Nat
            (mul (mul c aa) (mul bb dd))
            (mul (mul c aa) (mul dd bb))
            (mul (mul c dd) (mul aa bb))
            (mul_inner_comm (mul c aa) bb dd)
            (mul_shuffle c aa dd bb))
          (leq_nat_mul_right_mono_any (mul d cc) (mul c dd) (mul aa bb) wz))))

theorem leq_nat_add_mono
      (a : Nat)
      (b : Nat)
      (c : Nat)
      (d : Nat)
      (ab : Equal Bool (leq_nat a b) True)
      (cd : Equal Bool (leq_nat c d) True)
    : Equal Bool (leq_nat (add a c) (add b d)) True =
  (proof trans for leq_nat)
    (add a c)
    (add b c)
    (add b d)
    (leq_nat_add_right_mono a b c ab)
    (leq_rewrite
      (add b c)
      (add c b)
      (add b d)
      (add d b)
      ((proof comm for add) b c)
      ((proof comm for add) b d)
      (leq_nat_add_right_mono c d b cd))

theorem leq_nat_add_right_mono
      (a : Nat) (b : Nat) (c : Nat) (ab : Equal Bool (leq_nat a b) True)
    : Equal Bool (leq_nat (add a c) (add b c)) True =
  match c {
    Zero ↦ ab;
    Suc rest ↦ leq_nat_add_right_mono a b rest ab
  }

theorem leq_nat_mul_left_mono_any
      (k : Nat) (a : Nat) (b : Nat) (ab : Equal Bool (leq_nat a b) True)
    : Equal Bool (leq_nat (mul k a) (mul k b)) True =
  leq_rewrite
    (mul k a)
    (mul a k)
    (mul k b)
    (mul b k)
    ((proof comm for mul) k a)
    ((proof comm for mul) k b)
    (leq_nat_mul_right_mono_any a b k ab)

theorem leq_nat_mul_right_mono_any
      (a : Nat) (b : Nat) (k : Nat) (ab : Equal Bool (leq_nat a b) True)
    : Equal Bool (leq_nat (mul a k) (mul b k)) True =
  match k {
    Zero ↦ Proved;
    Suc rest ↦ leq_nat_mul_right_mono a b rest ab
  }

theorem leq_rewrite
      (l : Nat)
      (lp : Nat)
      (r : Nat)
      (rp : Nat)
      (hl : Equal Nat l lp)
      (hr : Equal Nat r rp)
      (h : Equal Bool (leq_nat lp rp) True)
    : Equal Bool (leq_nat l r) True =
  trans Bool (leq_nat l r) (leq_nat lp rp) True (leq_cong_two l lp r rp hl hr) h

theorem mul_inner_comm
      (p : Nat) (u : Nat) (v : Nat)
    : Equal Nat (mul p (mul u v)) (mul p (mul v u)) =
  mul_cong2 p p (mul u v) (mul v u) Refl ((proof comm for mul) u v)
```

## 5. Design notes

The positive product denominator is assembled from its predecessor. Thus the
constructor never accepts a denominator equal to zero, even when multiplication
forms an unreduced representation. Equivalence and all algebraic equations
compare values; different carrier spellings are not asserted to have structural
`Equal Rational` proofs. The Boolean order uses the canonical Nat relation.
Value-equivalent but structurally distinct inputs may yield different stored
spellings under
min/max when argument order changes; no structural commutativity is claimed.

## 6. References

- **Wikipedia** — [Rational number](https://en.wikipedia.org/wiki/Rational_number)
  — cross-product equality and the distinction between a fraction's spelling
  and its value.
- **Ken specification** —
  [Exact non-negative rational values](../../../../spec/50-stdlib/64-rational.md)
  — normative carrier, algebra and trust boundaries for this package.
- **Ken specification** —
  [Exact numerical intervals](../../../../spec/50-stdlib/68-numerical.md)
  — monotonicity and min/max order obligations used by intervals.

## 7. Trust & derivation

1. **Public API.** The exported constructor, projections, value constants,
   natural embedding, equivalence, four arithmetic/order operations and the
   checked equations and order laws of specification §§1–2.2 appear above.
   `rational_add_mono` and `rational_mul_mono` supply checked two-input
   monotonicity. `rational_min` and `rational_max` add six attached order
   proofs required by interval hull and intersection. `rational_sub` adds
   self-zero, ordered add-cancellation and ordered two-input distribution
   under spec 68 §15; `rational_sub_mono` supplies the two-sided order law.
2. **Source map.** [Definition](#2-definition) lists the exported values and
   laws; [Laws & proofs](#4-laws--proofs) explains and carries the private
   derivation, including the cross-product monotonicity helpers;
   [Using it](#3-using-it) compares distinct spellings.
3. **Derivation.** The carrier uses `Nat`; arithmetic reuses the checked Nat
   operations, algebraic laws, positive cancellation and canonical order.
   Order transitivity additionally uses the Nat positive-factor monotonicity
   and cancellation laws; totality uses the canonical Nat totality proof.
   Paired-input monotonicity uses only checked Nat add and multiply laws.
   Comparison selection uses rational order reflexivity and totality, with
   private pick helpers connecting each Boolean branch to its bounds.
   Subtraction uses Nat's checked truncated `sub`, right-add cancellation,
   multiplication distribution and both directions of subtraction order.
   The Rational sum cancellation, factor rearrangements and cross-product
   monotonicity stage remain private.
4. **Trusted-base delta.** Zero: all exported theorems carry checked proof
   terms. Nat and equality imports retain their existing trusted base; this
   package adds no `Axiom`, postulate, primitive or floating point.
5. **Validation.** The
   [rational seed](../../../../conformance/stdlib/numeric/seed-rational.md)
   tests independent closed values; its cases do not replace the generic
   checked proof terms.
