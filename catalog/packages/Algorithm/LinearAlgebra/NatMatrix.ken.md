# Dimension-safe natural matrices

Natural vectors and row-major matrices carry dimensions in their `Vec` types;
all arithmetic is exact, and the five algebraic laws below have checked proofs.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

An indexed vector makes shape mismatch a type error rather than a truncated
computation. A matrix is a vector of rows whose common column length is also
indexed. Products consequently accept only matching inner dimensions while
zero-row and zero-column matrices use the same operations as nonempty ones.

## 2. Definition

The canonical operations follow their contract: componentwise addition,
scaling and dot product, followed by matrix-vector multiplication, transpose,
matrix product and diagonal identity. The named private stages build columns
and row products; no second matrix carrier or numeric primitive is needed.

```ken
import Data.Vector.Vector (Vec, VNil, VCons, map, zip_with, vec_zero_vnil)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l)

fn prepend (n : Nat) (x : Nat) (column : Vec Nat n) : Vec Nat (Suc n) = VCons Nat n x column

fn prepend_zero (n : Nat) (row : Vec Nat n) : Vec Nat (Suc n) = VCons Nat n Zero row

fn repeat (a : Type) (n : Nat) (x : a) : Vec a n =
  match n {
    Zero ↦ VNil a;
    Suc k ↦ VCons a k x (repeat a k x)
  }

pub fn vector_add (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n =
  zip_with Nat Nat Nat n add xs ys

pub fn vector_scale (n : Nat) (factor : Nat) (xs : Vec Nat n) : Vec Nat n =
  map Nat Nat n (mul factor) xs

pub fn dot (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Nat =
  match xs {
    VNil ↦ Zero;
    VCons k x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦ add (mul x y) (dot k tail_xs tail_ys)
      }
  }

pub fn matrix_vector
      (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m) (vector : Vec Nat n)
    : Vec Nat m =
  map (Vec Nat n) Nat m (λrow. dot n row vector) rows

pub fn transpose (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m) : Vec (Vec Nat m) n =
  match rows {
    VNil ↦ repeat (Vec Nat Zero) n (VNil Nat);
    VCons k row tail_rows ↦
      zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row (transpose k n tail_rows)
  }

fn row_product_columns
      (n : Nat) (p : Nat) (row : Vec Nat n) (columns : Vec (Vec Nat n) p)
    : Vec Nat p =
  map (Vec Nat n) Nat p (λcolumn. dot n row column) columns

fn row_product (n : Nat) (p : Nat) (right : Vec (Vec Nat p) n) (row : Vec Nat n) : Vec Nat p =
  row_product_columns n p row (transpose n p right)

pub fn matrix_product
      (m : Nat) (n : Nat) (p : Nat) (left : Vec (Vec Nat n) m) (right : Vec (Vec Nat p) n)
    : Vec (Vec Nat p) m =
  map (Vec Nat n) (Vec Nat p) m (row_product n p right) left

pub fn identity (n : Nat) : Vec (Vec Nat n) n =
  match n {
    Zero ↦ VNil (Vec Nat Zero);
    Suc k ↦
      VCons
        (Vec Nat (Suc k))
        k
        (VCons Nat k (Suc Zero) (repeat Nat k Zero))
        (map (Vec Nat k) (Vec Nat (Suc k)) k (prepend_zero k) (identity k))
  }

pub theorem dot_comm
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal Nat (dot n xs ys) (dot n ys xs) =
  match xs {
    VNil ↦
      J
        (λys2 _. Equal Nat (dot Zero (VNil Nat) ys2) (dot Zero ys2 (VNil Nat)))
        Proved
        (vec_zero_vnil Nat ys);
    VCons k x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          trans
            Nat
            (add (mul x y) (dot k tail_xs tail_ys))
            (add (mul y x) (dot k tail_xs tail_ys))
            (add (mul y x) (dot k tail_ys tail_xs))
            (cong
              Nat
              Nat
              (mul x y)
              (mul y x)
              (λv. add v (dot k tail_xs tail_ys))
              ((proof comm for mul) x y))
            (cong
              Nat
              Nat
              (dot k tail_xs tail_ys)
              (dot k tail_ys tail_xs)
              (add (mul y x))
              (dot_comm k tail_xs tail_ys))
      }
  }

theorem swap_middle
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (add (add a b) c) (add (add a c) b) =
  trans
    Nat
    (add (add a b) c)
    (add a (add b c))
    (add (add a c) b)
    (sym Nat (add a (add b c)) (add (add a b) c) ((proof assoc for add) a b c))
    (trans
      Nat
      (add a (add b c))
      (add a (add c b))
      (add (add a c) b)
      (cong Nat Nat (add b c) (add c b) (λz. add a z) ((proof comm for add) b c))
      ((proof assoc for add) a c b))

theorem add_shuffle
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
      (add (add (add a c) b) d)
      (add (add a c) (add b d))
      (cong Nat Nat (add (add a b) c) (add (add a c) b) (λz. add z d) (swap_middle a b c))
      (sym
        Nat
        (add (add a c) (add b d))
        (add (add (add a c) b) d)
        ((proof assoc for add) (add a c) b d)))

pub theorem dot_distrib
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
    : Equal Nat (dot n (vector_add n xs ys) zs) (add (dot n xs zs) (dot n ys zs)) =
  match xs {
    VNil ↦
      J (λys2 _. Equal Nat Zero (add Zero (dot Zero ys2 zs))) Proved (vec_zero_vnil Nat ys);
    VCons k x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          match zs {
            VCons _ z tail_zs ↦
              let
                left_factor = mul x z;
                right_factor = mul y z;
                left_tail_dot = dot k tail_xs tail_zs;
                right_tail_dot = dot k tail_ys tail_zs;
                combined_tail_dot = dot k (vector_add k tail_xs tail_ys) tail_zs
              in
                trans
                  Nat
                  (add (mul (add x y) z) combined_tail_dot)
                  (add (add left_factor right_factor) combined_tail_dot)
                  (add (add left_factor left_tail_dot) (add right_factor right_tail_dot))
                  (cong
                    Nat
                    Nat
                    (mul (add x y) z)
                    (add left_factor right_factor)
                    (λv. add v combined_tail_dot)
                    (mul_add_distrib_l x y z))
                  (trans
                    Nat
                    (add (add left_factor right_factor) combined_tail_dot)
                    (add (add left_factor right_factor) (add left_tail_dot right_tail_dot))
                    (add (add left_factor left_tail_dot) (add right_factor right_tail_dot))
                    (cong
                      Nat
                      Nat
                      combined_tail_dot
                      (add left_tail_dot right_tail_dot)
                      (add (add left_factor right_factor))
                      (dot_distrib k tail_xs tail_ys tail_zs))
                    (add_shuffle left_factor right_factor left_tail_dot right_tail_dot))
          }
      }
  }

theorem transpose_zip_cons_empty
      (k : Nat)
    : Equal
        (Vec (Vec Nat Zero) (Suc k))
        (transpose
          Zero
          (Suc k)
          (zip_with
            Nat
            (Vec Nat k)
            (Vec Nat (Suc k))
            Zero
            (prepend k)
            (VNil Nat)
            (VNil (Vec Nat k))))
        (VCons (Vec Nat Zero) k (VNil Nat) (transpose Zero k (VNil (Vec Nat k)))) =
  Refl

theorem transpose_zip_cons
      (k : Nat) (n : Nat) (row : Vec Nat n) (columns : Vec (Vec Nat k) n)
    : Equal
        (Vec (Vec Nat n) (Suc k))
        (transpose
          n
          (Suc k)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row columns))
        (VCons (Vec Nat n) k row (transpose n k columns)) =
  match row {
    VNil ↦
      J
        (λcolumns2 _.
          Equal
            (Vec (Vec Nat Zero) (Suc k))
            (transpose
              Zero
              (Suc k)
              (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) Zero (prepend k) (VNil Nat) columns2))
            (VCons (Vec Nat Zero) k (VNil Nat) (transpose Zero k columns2)))
        (transpose_zip_cons_empty k)
        (vec_zero_vnil (Vec Nat k) columns);
    VCons t x rest_row ↦
      match columns {
        VCons _ col rest_columns ↦
          let
            old_tail =
              transpose
                t
                (Suc k)
                (zip_with
                  Nat
                  (Vec Nat k)
                  (Vec Nat (Suc k))
                  t
                  (prepend k)
                  rest_row
                  rest_columns);
            new_tail = VCons (Vec Nat t) k rest_row (transpose t k rest_columns);
            head_column = VCons Nat k x col
          in
            cong
              (Vec (Vec Nat t) (Suc k))
              (Vec (Vec Nat (Suc t)) (Suc k))
              old_tail
              new_tail
              (λtail.
                zip_with Nat (Vec Nat t) (Vec Nat (Suc t)) (Suc k) (prepend t) head_column tail)
              (transpose_zip_cons k t rest_row rest_columns)
      }
  }

theorem transpose_zero_repeat
      (n : Nat)
    : Equal
        (Vec (Vec Nat n) Zero)
        (transpose n Zero (repeat (Vec Nat Zero) n (VNil Nat)))
        (VNil (Vec Nat n)) =
  match n {
    Zero ↦ Proved;
    Suc k ↦ Proved
  }

pub theorem transpose_invol
      (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m)
    : Equal (Vec (Vec Nat n) m) (transpose n m (transpose m n rows)) rows =
  match rows {
    VNil ↦ transpose_zero_repeat n;
    VCons k row rest ↦
      trans
        (Vec (Vec Nat n) (Suc k))
        (transpose n (Suc k) (transpose (Suc k) n (VCons (Vec Nat n) k row rest)))
        (VCons (Vec Nat n) k row (transpose n k (transpose k n rest)))
        (VCons (Vec Nat n) k row rest)
        (transpose_zip_cons k n row (transpose k n rest))
        (cong
          (Vec (Vec Nat n) k)
          (Vec (Vec Nat n) (Suc k))
          (transpose n k (transpose k n rest))
          rest
          (VCons (Vec Nat n) k row)
          (transpose_invol k n rest))
  }

theorem dot_zero_left
      (n : Nat) (xs : Vec Nat n)
    : Equal Nat (dot n (repeat Nat n Zero) xs) Zero =
  match xs {
    VNil ↦ Proved;
    VCons k x tail ↦
      trans
        Nat
        (add (mul Zero x) (dot k (repeat Nat k Zero) tail))
        (add Zero (dot k (repeat Nat k Zero) tail))
        Zero
        (cong
          Nat
          Nat
          (mul Zero x)
          Zero
          (λv. add v (dot k (repeat Nat k Zero) tail))
          ((proof zero_l for mul) x))
        (trans
          Nat
          (add Zero (dot k (repeat Nat k Zero) tail))
          (dot k (repeat Nat k Zero) tail)
          Zero
          ((proof zero_l for add) (dot k (repeat Nat k Zero) tail))
          (dot_zero_left k tail))
  }

theorem dot_one_head
      (n : Nat) (x : Nat) (tail : Vec Nat n)
    : Equal Nat
        (dot (Suc n) (VCons Nat n (Suc Zero) (repeat Nat n Zero)) (VCons Nat n x tail))
        x =
  trans
    Nat
    (add (mul (Suc Zero) x) (dot n (repeat Nat n Zero) tail))
    (add x (dot n (repeat Nat n Zero) tail))
    x
    (cong
      Nat
      Nat
      (mul (Suc Zero) x)
      x
      (λv. add v (dot n (repeat Nat n Zero) tail))
      ((proof one_l for mul) x))
    (cong Nat Nat (dot n (repeat Nat n Zero) tail) Zero (add x) (dot_zero_left n tail))

theorem dot_zero_prefix
      (n : Nat) (x : Nat) (row : Vec Nat n) (tail : Vec Nat n)
    : Equal Nat (dot (Suc n) (VCons Nat n Zero row) (VCons Nat n x tail)) (dot n row tail) =
  trans
    Nat
    (add (mul Zero x) (dot n row tail))
    (add Zero (dot n row tail))
    (dot n row tail)
    (cong Nat Nat (mul Zero x) Zero (λv. add v (dot n row tail)) ((proof zero_l for mul) x))
    ((proof zero_l for add) (dot n row tail))

theorem matrix_vector_zero_prefix
      (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m) (x : Nat) (tail : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector
          m
          (Suc n)
          (map (Vec Nat n) (Vec Nat (Suc n)) m (prepend_zero n) rows)
          (VCons Nat n x tail))
        (matrix_vector m n rows tail) =
  match rows {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        left_tail =
          matrix_vector
            k
            (Suc n)
            (map (Vec Nat n) (Vec Nat (Suc n)) k (prepend_zero n) rest)
            (VCons Nat n x tail);
        right_tail = matrix_vector k n rest tail;
        old_head = dot (Suc n) (VCons Nat n Zero row) (VCons Nat n x tail);
        new_head = dot n row tail
      in
        trans
          (Vec Nat (Suc k))
          (VCons Nat k old_head left_tail)
          (VCons Nat k new_head left_tail)
          (VCons Nat k new_head right_tail)
          (cong
            Nat
            (Vec Nat (Suc k))
            old_head
            new_head
            (λv. VCons Nat k v left_tail)
            (dot_zero_prefix n x row tail))
          (cong
            (Vec Nat k)
            (Vec Nat (Suc k))
            left_tail
            right_tail
            (VCons Nat k new_head)
            (matrix_vector_zero_prefix k n rest x tail))
  }

pub theorem vector_identity
      (n : Nat) (v : Vec Nat n)
    : Equal (Vec Nat n) (matrix_vector n n (identity n) v) v =
  match v {
    VNil ↦ Proved;
    VCons k x tail ↦
      let
        head = dot (Suc k) (VCons Nat k (Suc Zero) (repeat Nat k Zero)) (VCons Nat k x tail);
        rest_rows = map (Vec Nat k) (Vec Nat (Suc k)) k (prepend_zero k) (identity k);
        old_tail = matrix_vector k (Suc k) rest_rows (VCons Nat k x tail);
        new_tail = matrix_vector k k (identity k) tail
      in
        trans
          (Vec Nat (Suc k))
          (VCons Nat k head old_tail)
          (VCons Nat k x new_tail)
          (VCons Nat k x tail)
          (trans
            (Vec Nat (Suc k))
            (VCons Nat k head old_tail)
            (VCons Nat k x old_tail)
            (VCons Nat k x new_tail)
            (cong
              Nat
              (Vec Nat (Suc k))
              head
              x
              (λv2. VCons Nat k v2 old_tail)
              (dot_one_head k x tail))
            (cong
              (Vec Nat k)
              (Vec Nat (Suc k))
              old_tail
              new_tail
              (VCons Nat k x)
              (matrix_vector_zero_prefix k k (identity k) x tail)))
          (cong
            (Vec Nat k)
            (Vec Nat (Suc k))
            new_tail
            tail
            (VCons Nat k x)
            (vector_identity k tail))
  }

theorem transpose_zero_prefix_rows
      (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat m) (Suc n))
        (transpose m (Suc n) (map (Vec Nat n) (Vec Nat (Suc n)) m (prepend_zero n) rows))
        (VCons (Vec Nat m) n (repeat Nat m Zero) (transpose m n rows)) =
  match rows {
    VNil ↦ Refl;
    VCons k row rest ↦
      let
        old_tail =
          transpose k (Suc n) (map (Vec Nat n) (Vec Nat (Suc n)) k (prepend_zero n) rest);
        new_tail = VCons (Vec Nat k) n (repeat Nat k Zero) (transpose k n rest);
        head_row = VCons Nat n Zero row
      in
        cong
          (Vec (Vec Nat k) (Suc n))
          (Vec (Vec Nat (Suc k)) (Suc n))
          old_tail
          new_tail
          (λtail. zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) (Suc n) (prepend k) head_row tail)
          (transpose_zero_prefix_rows k n rest)
  }

theorem zip_zero_map
      (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat (Suc n)) m)
        (zip_with Nat (Vec Nat n) (Vec Nat (Suc n)) m (prepend n) (repeat Nat m Zero) rows)
        (map (Vec Nat n) (Vec Nat (Suc n)) m (prepend_zero n) rows) =
  match rows {
    VNil ↦ Proved;
    VCons k row rest ↦
      cong
        (Vec (Vec Nat (Suc n)) k)
        (Vec (Vec Nat (Suc n)) (Suc k))
        (zip_with Nat (Vec Nat n) (Vec Nat (Suc n)) k (prepend n) (repeat Nat k Zero) rest)
        (map (Vec Nat n) (Vec Nat (Suc n)) k (prepend_zero n) rest)
        (VCons (Vec Nat (Suc n)) k (VCons Nat n Zero row))
        (zip_zero_map k n rest)
  }

theorem transpose_identity
      (n : Nat)
    : Equal (Vec (Vec Nat n) n) (transpose n n (identity n)) (identity n) =
  match n {
    Zero ↦ Proved;
    Suc k ↦
      let
        first = VCons Nat k (Suc Zero) (repeat Nat k Zero);
        old_tail =
          transpose
            k
            (Suc k)
            (map (Vec Nat k) (Vec Nat (Suc k)) k (prepend_zero k) (identity k));
        new_tail = VCons (Vec Nat k) k (repeat Nat k Zero) (transpose k k (identity k));
        zipped_old =
          zip_with
            Nat
            (Vec Nat k)
            (Vec Nat (Suc k))
            k
            (prepend k)
            (repeat Nat k Zero)
            (transpose k k (identity k));
        zipped_mid =
          zip_with
            Nat
            (Vec Nat k)
            (Vec Nat (Suc k))
            k
            (prepend k)
            (repeat Nat k Zero)
            (identity k);
        mapped = map (Vec Nat k) (Vec Nat (Suc k)) k (prepend_zero k) (identity k);
        start_value =
          zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) (Suc k) (prepend k) first old_tail;
        after_zero = VCons (Vec Nat (Suc k)) k first zipped_old;
        after_ih = VCons (Vec Nat (Suc k)) k first zipped_mid;
        wanted = VCons (Vec Nat (Suc k)) k first mapped
      in
        trans
          (Vec (Vec Nat (Suc k)) (Suc k))
          start_value
          after_zero
          wanted
          (cong
            (Vec (Vec Nat k) (Suc k))
            (Vec (Vec Nat (Suc k)) (Suc k))
            old_tail
            new_tail
            (λtail. zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) (Suc k) (prepend k) first tail)
            (transpose_zero_prefix_rows k k (identity k)))
          (trans
            (Vec (Vec Nat (Suc k)) (Suc k))
            after_zero
            after_ih
            wanted
            (cong
              (Vec (Vec Nat (Suc k)) k)
              (Vec (Vec Nat (Suc k)) (Suc k))
              zipped_old
              zipped_mid
              (VCons (Vec Nat (Suc k)) k first)
              (cong
                (Vec (Vec Nat k) k)
                (Vec (Vec Nat (Suc k)) k)
                (transpose k k (identity k))
                (identity k)
                (λv.
                  zip_with
                    Nat
                    (Vec Nat k)
                    (Vec Nat (Suc k))
                    k
                    (prepend k)
                    (repeat Nat k Zero)
                    v)
                (transpose_identity k)))
            (cong
              (Vec (Vec Nat (Suc k)) k)
              (Vec (Vec Nat (Suc k)) (Suc k))
              zipped_mid
              mapped
              (VCons (Vec Nat (Suc k)) k first)
              (zip_zero_map k k (identity k))))
  }

theorem map_cong
      (a : Type)
      (b : Type)
      (n : Nat)
      (f : a → b)
      (g : a → b)
      (agree : (x : a) → Equal b (f x) (g x))
      (xs : Vec a n)
    : Equal (Vec b n) (map a b n f xs) (map a b n g xs) =
  match xs {
    VNil ↦ Proved;
    VCons k x tail ↦
      let
        ltail = map a b k f tail;
        rtail = map a b k g tail
      in
        trans
          (Vec b (Suc k))
          (VCons b k (f x) ltail)
          (VCons b k (g x) ltail)
          (VCons b k (g x) rtail)
          (cong b (Vec b (Suc k)) (f x) (g x) (λv. VCons b k v ltail) (agree x))
          (cong
            (Vec b k)
            (Vec b (Suc k))
            ltail
            rtail
            (VCons b k (g x))
            (map_cong a b k f g agree tail))
  }

theorem row_right_identity
      (n : Nat) (row : Vec Nat n)
    : Equal (Vec Nat n) (row_product n n (identity n) row) row =
  let
    cols = transpose n n (identity n);
    f : Vec Nat n → Nat = λcolumn. dot n row column;
    g : Vec Nat n → Nat = λcolumn. dot n column row;
    with_cols = map (Vec Nat n) Nat n f cols;
    with_rows = map (Vec Nat n) Nat n f (identity n);
    reversed = map (Vec Nat n) Nat n g (identity n)
  in
    trans
      (Vec Nat n)
      with_cols
      with_rows
      row
      (cong
        (Vec (Vec Nat n) n)
        (Vec Nat n)
        cols
        (identity n)
        (map (Vec Nat n) Nat n f)
        (transpose_identity n))
      (trans
        (Vec Nat n)
        with_rows
        reversed
        row
        (map_cong (Vec Nat n) Nat n f g (λcol. dot_comm n row col) (identity n))
        (vector_identity n row))

pub theorem matrix_identity_right
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m)
    : Equal (Vec (Vec Nat n) m) (matrix_product m n n a (identity n)) a =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        old_head = row_product n n (identity n) row;
        old_tail = matrix_product k n n rest (identity n)
      in
        trans
          (Vec (Vec Nat n) (Suc k))
          (VCons (Vec Nat n) k old_head old_tail)
          (VCons (Vec Nat n) k row old_tail)
          (VCons (Vec Nat n) k row rest)
          (cong
            (Vec Nat n)
            (Vec (Vec Nat n) (Suc k))
            old_head
            row
            (λv. VCons (Vec Nat n) k v old_tail)
            (row_right_identity n row))
          (cong
            (Vec (Vec Nat n) k)
            (Vec (Vec Nat n) (Suc k))
            old_tail
            rest
            (VCons (Vec Nat n) k row)
            (matrix_identity_right k n rest))
  }
```

## 3. Using it

A one-component vector is a row, a column or both; its dot product has one
contribution. Here the identity has one row containing one.

```ken example
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (dot, identity)

const one : Nat = Suc Zero

const singleton : Vec Nat one = VCons Nat Zero one (VNil Nat)

theorem singleton_dot : Equal Nat (dot one singleton singleton) one = Proved

theorem singleton_identity
    : Equal
        (Vec (Vec Nat one) one)
        (identity one)
        (VCons (Vec Nat one) Zero singleton (VNil (Vec Nat one))) =
  Refl
```

## 4. Laws & proofs

`dot_comm` transports the right empty vector to `VNil`, then commutes each
paired product and applies the tail induction. `dot_distrib` uses the checked
natural-number distribution law, its induction hypothesis and a private
four-term addition shuffle. The shuffle derives only from checked `add`
associativity and commutativity; it does not add a shared export.

`transpose_invol` proves the column-prepend step then inducts over rows.
`vector_identity` reduces a diagonal head dot to its head value, proves that
zero-prefixed trailing rows ignore the added coordinate, and recurses.
`matrix_identity_right` first proves the diagonal matrix is unchanged by
transpose, applies `dot_comm` pointwise to each right-identity row, and lifts
the row proof across the matrix. These are generic, indexed proof terms; the
closed example only illustrates their operations.

## 5. Design notes

A matrix is `Vec (Vec Nat n) m`, not a list plus an unchecked width. `transpose`
keeps columns and rows in source order. `identity` places `Suc Zero` on the
diagonal and `Zero` elsewhere. The laws make no claim about left matrix
identity or associativity of matrix products, which require separate generic
proofs. A rational-valued or approximate matrix would require a different
statement of equality and is not implied by natural-number arithmetic.

## 6. References

- [Matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
  — Wikipedia; orientation for inner dimensions, ordered products and identity.
- [Vector space](https://en.wikipedia.org/wiki/Vector_space) — Wikipedia;
  background on operations over scalars, not a claim that `Nat` is a field.

## 7. Trust & derivation

The [linear-algebra contract](../../../../spec/50-stdlib/65-linear-algebra.md)
§1 and its [seed](../../../../conformance/stdlib/linear-algebra/seed-nat-matrix.md)
own the twelve public names and nine behavioral cases. The module reuses the
published `Data.Vector.Vector` carrier and natural arithmetic. `repeat`,
column prepending, row products and proof bridges remain private. All five
public theorems have checked bodies. This package adds no Axiom, postulate,
primitive, `Float` or trusted-base entry; dependencies retain their own trust
posture. The root-loaded acceptance target checks generic imports, closed
values, refusal of a wrong inner dimension and the exact trusted-identity set.
