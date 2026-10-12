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

## 2. Definition

The second constructor field is the denominator's predecessor: `MkRational n k`
represents `n/(Suc k)`. All exported calculations preserve that positivity.

```ken
import Core.Logic.Transport (sym, trans, cong)

import Core.Classes.LawfulClasses (leq_nat)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l, mul_add_distrib_r)

import Data.Numeric.Nat.Order (lt_nat, leq_nat_mul_right_mono, leq_nat_mul_right_cancel)

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

Commutativity and multiplicative associativity lift the matching natural-number
algebra to both numerator and denominator. Additive associativity expands the
cross sums and re-associates three natural summands. Left distributivity
expands the numerator and refactors the two resulting products around their
shared denominator. Every final equation is value equivalence, not structural
identity of two carrier spellings.

```ken
fn positive_product_tail (a : Nat) (b : Nat) : Nat = add (mul (Suc a) b) a

theorem positive_product_factor
      (bx : Nat) (by : Nat)
    : Equal Nat (Suc (positive_product_tail bx by)) (mul (Suc bx) (Suc by)) =
  Refl

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
```

## 5. Design notes

The positive product denominator is assembled from its predecessor. Thus the
constructor never accepts a denominator equal to zero, even when multiplication
forms an unreduced representation. Equivalence and all algebraic equations
compare values; different carrier spellings are not asserted to have structural
`Equal Rational` proofs. The Boolean order uses the canonical Nat relation.

## 6. References

- **Wikipedia** — [Rational number](https://en.wikipedia.org/wiki/Rational_number)
  — cross-product equality and the distinction between a fraction's spelling
  and its value.
- **Ken specification** —
  [Exact non-negative rational values](../../../../spec/50-stdlib/64-rational.md)
  — normative carrier, algebra and trust boundaries for this package.

## 7. Trust & derivation

1. **Public API.** The exported constructor, projections, value constants,
   natural embedding, equivalence, three arithmetic/order operations and the
   checked equations of specification §§1–2.1 appear above.
2. **Source map.** [Definition](#2-definition) lists the exported values and
   laws; [Laws & proofs](#4-laws--proofs) explains and carries the private
   derivation; [Using it](#3-using-it) compares distinct spellings.
3. **Derivation.** The carrier uses `Nat`; arithmetic reuses the checked Nat
   operations, algebraic laws, positive cancellation and canonical order.
4. **Trusted-base delta.** Zero: all exported theorems carry checked proof
   terms. Nat and equality imports retain their existing trusted base; this
   package adds no `Axiom`, postulate, primitive or floating point.
5. **Validation.** The
   [rational seed](../../../../conformance/stdlib/numeric/seed-rational.md)
   tests independent closed values; its cases do not replace the generic
   checked proof terms.
