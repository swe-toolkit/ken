# Dimension-safe natural matrices

Natural vectors and row-major matrices carry dimensions in their `Vec` types;
all arithmetic is exact, and twenty-one algebraic laws have checked proofs.

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
matrix product and diagonal identity. Row-wise matrix addition and its six
public laws follow those operations and laws, alongside two vector sum laws.
The named private stages build columns and row products; no second matrix
carrier or numeric primitive is needed.

```ken
import Data.Vector.Vector (Vec, VNil, VCons, map, zip_with, vec_zero_vnil)

import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l, mul_add_distrib_r)

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

pub theorem dot_scale
      (n : Nat) (x : Nat) (u : Vec Nat n) (c : Vec Nat n)
    : Equal Nat (dot n (vector_scale n x u) c) (mul x (dot n u c)) =
  match u {
    VNil ↦ Proved;
    VCons k y tail_u ↦
      match c {
        VCons _ z tail_c ↦
          let
            tail_dot = dot k tail_u tail_c;
            scaled_tail_dot = dot k (vector_scale k x tail_u) tail_c
          in
            trans
              Nat
              (add (mul (mul x y) z) scaled_tail_dot)
              (add (mul x (mul y z)) scaled_tail_dot)
              (mul x (add (mul y z) tail_dot))
              (cong
                Nat
                Nat
                (mul (mul x y) z)
                (mul x (mul y z))
                (λv. add v scaled_tail_dot)
                (sym Nat (mul x (mul y z)) (mul (mul x y) z) ((proof assoc for mul) x y z)))
              (trans
                Nat
                (add (mul x (mul y z)) scaled_tail_dot)
                (add (mul x (mul y z)) (mul x tail_dot))
                (mul x (add (mul y z) tail_dot))
                (cong
                  Nat
                  Nat
                  scaled_tail_dot
                  (mul x tail_dot)
                  (add (mul x (mul y z)))
                  (dot_scale k x tail_u tail_c))
                (sym
                  Nat
                  (mul x (add (mul y z) tail_dot))
                  (add (mul x (mul y z)) (mul x tail_dot))
                  (mul_add_distrib_r x (mul y z) tail_dot)))
      }
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

pub theorem matrix_identity_left
      (m : Nat) (n : Nat) (a : Vec (Vec Nat m) n)
    : Equal (Vec (Vec Nat m) n) (matrix_product n n m (identity n) a) a =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        whole = VCons (Vec Nat m) k row rest;
        old_head = row_product (Suc k) m whole (VCons Nat k (Suc Zero) (repeat Nat k Zero));
        old_tail =
          map
            (Vec Nat (Suc k))
            (Vec Nat m)
            k
            (row_product (Suc k) m whole)
            (map (Vec Nat k) (Vec Nat (Suc k)) k (prepend_zero k) (identity k));
        mid_tail = matrix_product k k m (identity k) rest
      in
        trans
          (Vec (Vec Nat m) (Suc k))
          (VCons (Vec Nat m) k old_head old_tail)
          (VCons (Vec Nat m) k row old_tail)
          (VCons (Vec Nat m) k row rest)
          (cong
            (Vec Nat m)
            (Vec (Vec Nat m) (Suc k))
            old_head
            row
            (λv. VCons (Vec Nat m) k v old_tail)
            (left_head k m row (transpose k m rest)))
          (cong
            (Vec (Vec Nat m) k)
            (Vec (Vec Nat m) (Suc k))
            old_tail
            rest
            (VCons (Vec Nat m) k row)
            (trans
              (Vec (Vec Nat m) k)
              old_tail
              mid_tail
              rest
              (left_tail_rows k m row rest k (identity k))
              (matrix_identity_left m k rest)))
  }

pub theorem matrix_product_assoc
      (m : Nat)
      (n : Nat)
      (p : Nat)
      (q : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat p) n)
      (c : Vec (Vec Nat q) p)
    : Equal
        (Vec (Vec Nat q) m)
        (matrix_product m p q (matrix_product m n p a b) c)
        (matrix_product m n q a (matrix_product n p q b c)) =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        bc = matrix_product n p q b c;
        old_head = row_product p q c (row_product n p b row);
        new_head = row_product n q bc row;
        old_tail = matrix_product k p q (matrix_product k n p rest b) c;
        new_tail = matrix_product k n q rest bc
      in
        trans
          (Vec (Vec Nat q) (Suc k))
          (VCons (Vec Nat q) k old_head old_tail)
          (VCons (Vec Nat q) k new_head old_tail)
          (VCons (Vec Nat q) k new_head new_tail)
          (cong
            (Vec Nat q)
            (Vec (Vec Nat q) (Suc k))
            old_head
            new_head
            (λv. VCons (Vec Nat q) k v old_tail)
            (row_product_assoc n p q row b c))
          (cong
            (Vec (Vec Nat q) k)
            (Vec (Vec Nat q) (Suc k))
            old_tail
            new_tail
            (VCons (Vec Nat q) k new_head)
            (matrix_product_assoc k n p q rest b c))
  }

pub theorem matrix_vector_product
      (m : Nat)
      (n : Nat)
      (p : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat p) n)
      (x : Vec Nat p)
    : Equal
        (Vec Nat m)
        (matrix_vector m p (matrix_product m n p a b) x)
        (matrix_vector m n a (matrix_vector n p b x)) =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        old_head = dot p (row_product n p b row) x;
        new_head = dot n row (matrix_vector n p b x);
        old_tail = matrix_vector k p (matrix_product k n p rest b) x;
        new_tail = matrix_vector k n rest (matrix_vector n p b x)
      in
        trans
          (Vec Nat (Suc k))
          (VCons Nat k old_head old_tail)
          (VCons Nat k new_head old_tail)
          (VCons Nat k new_head new_tail)
          (cong
            Nat
            (Vec Nat (Suc k))
            old_head
            new_head
            (λv. VCons Nat k v old_tail)
            (dot_row_product n p row b x))
          (cong
            (Vec Nat k)
            (Vec Nat (Suc k))
            old_tail
            new_tail
            (VCons Nat k new_head)
            (matrix_vector_product k n p rest b x))
  }

pub theorem transpose_product
      (m : Nat) (n : Nat) (p : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n)
    : Equal
        (Vec (Vec Nat m) p)
        (transpose m p (matrix_product m n p a b))
        (matrix_product p n m (transpose n p b) (transpose m n a)) =
  match a {
    VNil ↦
      zero_width_repeat
        p
        (matrix_product p n Zero (transpose n p b) (transpose Zero n (VNil (Vec Nat n))));
    VCons k arow rest ↦
      let
        bt = transpose n p b;
        at_rest = transpose k n rest;
        head = row_product n p b arow;
        ih_lhs = transpose k p (matrix_product k n p rest b);
        ih_rhs = matrix_product p n k bt at_rest
      in
        trans
          (Vec (Vec Nat (Suc k)) p)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) p (prepend k) head ih_lhs)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) p (prepend k) head ih_rhs)
          (matrix_product p n (Suc k) bt (transpose (Suc k) n (VCons (Vec Nat n) k arow rest)))
          (cong
            (Vec (Vec Nat k) p)
            (Vec (Vec Nat (Suc k)) p)
            ih_lhs
            ih_rhs
            (λt. zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) p (prepend k) head t)
            (transpose_product k n p rest b))
          (zip_prepend_map k n p arow at_rest bt)
  }

pub fn matrix_add
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Vec (Vec Nat n) m =
  zip_with (Vec Nat n) (Vec Nat n) (Vec Nat n) m (vector_add n) a b

pub theorem matrix_product_distrib_left
      (m : Nat)
      (n : Nat)
      (p : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat p) n)
      (c : Vec (Vec Nat p) n)
    : Equal
        (Vec (Vec Nat p) m)
        (matrix_product m n p a (matrix_add n p b c))
        (matrix_add m p (matrix_product m n p a b) (matrix_product m n p a c)) =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        old_head = row_product n p (matrix_add n p b c) row;
        new_head = vector_add p (row_product n p b row) (row_product n p c row);
        old_tail = matrix_product k n p rest (matrix_add n p b c);
        new_tail = matrix_add k p (matrix_product k n p rest b) (matrix_product k n p rest c)
      in
        trans
          (Vec (Vec Nat p) (Suc k))
          (VCons (Vec Nat p) k old_head old_tail)
          (VCons (Vec Nat p) k new_head old_tail)
          (VCons (Vec Nat p) k new_head new_tail)
          (cong
            (Vec Nat p)
            (Vec (Vec Nat p) (Suc k))
            old_head
            new_head
            (λv. VCons (Vec Nat p) k v old_tail)
            (row_product_add n p row b c))
          (cong
            (Vec (Vec Nat p) k)
            (Vec (Vec Nat p) (Suc k))
            old_tail
            new_tail
            (VCons (Vec Nat p) k new_head)
            (matrix_product_distrib_left k n p rest b c))
  }

pub theorem matrix_product_distrib_right
      (m : Nat)
      (n : Nat)
      (p : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat n) m)
      (c : Vec (Vec Nat p) n)
    : Equal
        (Vec (Vec Nat p) m)
        (matrix_product m n p (matrix_add m n a b) c)
        (matrix_add m p (matrix_product m n p a c) (matrix_product m n p b c)) =
  match a {
    VNil ↦ Proved;
    VCons k arow arest ↦
      match b {
        VCons _ brow brest ↦
          let
            old_head = row_product n p c (vector_add n arow brow);
            new_head = vector_add p (row_product n p c arow) (row_product n p c brow);
            old_tail = matrix_product k n p (matrix_add k n arest brest) c;
            new_tail =
              matrix_add k p (matrix_product k n p arest c) (matrix_product k n p brest c)
          in
            trans
              (Vec (Vec Nat p) (Suc k))
              (VCons (Vec Nat p) k old_head old_tail)
              (VCons (Vec Nat p) k new_head old_tail)
              (VCons (Vec Nat p) k new_head new_tail)
              (cong
                (Vec Nat p)
                (Vec (Vec Nat p) (Suc k))
                old_head
                new_head
                (λv. VCons (Vec Nat p) k v old_tail)
                (columns_add n p arow brow (transpose n p c)))
              (cong
                (Vec (Vec Nat p) k)
                (Vec (Vec Nat p) (Suc k))
                old_tail
                new_tail
                (VCons (Vec Nat p) k new_head)
                (matrix_product_distrib_right k n p arest brest c))
      }
  }

pub theorem matrix_vector_distrib
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m) (x : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n (matrix_add m n a b) x)
        (vector_add m (matrix_vector m n a x) (matrix_vector m n b x)) =
  match a {
    VNil ↦ Proved;
    VCons k arow arest ↦
      match b {
        VCons _ brow brest ↦
          let
            old_head = dot n (vector_add n arow brow) x;
            new_head = add (dot n arow x) (dot n brow x);
            old_tail = matrix_vector k n (matrix_add k n arest brest) x;
            new_tail = vector_add k (matrix_vector k n arest x) (matrix_vector k n brest x)
          in
            trans
              (Vec Nat (Suc k))
              (VCons Nat k old_head old_tail)
              (VCons Nat k new_head old_tail)
              (VCons Nat k new_head new_tail)
              (cong
                Nat
                (Vec Nat (Suc k))
                old_head
                new_head
                (λv. VCons Nat k v old_tail)
                (dot_distrib n arow brow x))
              (cong
                (Vec Nat k)
                (Vec Nat (Suc k))
                old_tail
                new_tail
                (VCons Nat k new_head)
                (matrix_vector_distrib k n arest brest x))
      }
  }

pub theorem transpose_add
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat m) n)
        (transpose m n (matrix_add m n a b))
        (matrix_add n m (transpose m n a) (transpose m n b)) =
  match a {
    VNil ↦
      zero_width_repeat
        n
        (matrix_add n Zero (transpose Zero n (VNil (Vec Nat n))) (transpose Zero n b));
    VCons k arow arest ↦
      match b {
        VCons _ brow brest ↦
          let
            at_rest = transpose k n arest;
            bt_rest = transpose k n brest;
            sum_row = vector_add n arow brow
          in
            trans
              (Vec (Vec Nat (Suc k)) n)
              (zip_with
                Nat
                (Vec Nat k)
                (Vec Nat (Suc k))
                n
                (prepend k)
                sum_row
                (transpose k n (matrix_add k n arest brest)))
              (zip_with
                Nat
                (Vec Nat k)
                (Vec Nat (Suc k))
                n
                (prepend k)
                sum_row
                (matrix_add n k at_rest bt_rest))
              (matrix_add
                n
                (Suc k)
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) arow at_rest)
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) brow bt_rest))
              (cong
                (Vec (Vec Nat k) n)
                (Vec (Vec Nat (Suc k)) n)
                (transpose k n (matrix_add k n arest brest))
                (matrix_add n k at_rest bt_rest)
                (λt. zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) sum_row t)
                (transpose_add k n arest brest))
              (zip_prepend_add k n arow brow at_rest bt_rest)
      }
  }

pub theorem matrix_add_comm
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_add m n b a) =
  match a {
    VNil ↦
      J
        (λb2 _.
          Equal
            (Vec (Vec Nat n) Zero)
            (matrix_add Zero n (VNil (Vec Nat n)) b2)
            (matrix_add Zero n b2 (VNil (Vec Nat n))))
        Proved
        (vec_zero_vnil (Vec Nat n) b);
    VCons k arow arest ↦
      match b {
        VCons _ brow brest ↦
          trans
            (Vec (Vec Nat n) (Suc k))
            (VCons (Vec Nat n) k (vector_add n arow brow) (matrix_add k n arest brest))
            (VCons (Vec Nat n) k (vector_add n brow arow) (matrix_add k n arest brest))
            (VCons (Vec Nat n) k (vector_add n brow arow) (matrix_add k n brest arest))
            (cong
              (Vec Nat n)
              (Vec (Vec Nat n) (Suc k))
              (vector_add n arow brow)
              (vector_add n brow arow)
              (λrow. VCons (Vec Nat n) k row (matrix_add k n arest brest))
              (vector_add_comm n arow brow))
            (cong
              (Vec (Vec Nat n) k)
              (Vec (Vec Nat n) (Suc k))
              (matrix_add k n arest brest)
              (matrix_add k n brest arest)
              (λrest. VCons (Vec Nat n) k (vector_add n brow arow) rest)
              (matrix_add_comm k n arest brest))
      }
  }

pub theorem matrix_add_assoc
      (m : Nat)
      (n : Nat)
      (a : Vec (Vec Nat n) m)
      (b : Vec (Vec Nat n) m)
      (c : Vec (Vec Nat n) m)
    : Equal
        (Vec (Vec Nat n) m)
        (matrix_add m n (matrix_add m n a b) c)
        (matrix_add m n a (matrix_add m n b c)) =
  match a {
    VNil ↦ Proved;
    VCons k arow arest ↦
      match b {
        VCons _ brow brest ↦
          match c {
            VCons _ crow crest ↦
              matrix_add_assoc_cons
                k
                n
                arow
                brow
                crow
                arest
                brest
                crest
                (matrix_add_assoc k n arest brest crest)
          }
      }
  }

pub theorem vector_add_comm
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal (Vec Nat n) (vector_add n xs ys) (vector_add n ys xs) =
  match xs {
    VNil ↦
      J
        (λys2 _.
          Equal
            (Vec Nat Zero)
            (vector_add Zero (VNil Nat) ys2)
            (vector_add Zero ys2 (VNil Nat)))
        Proved
        (vec_zero_vnil Nat ys);
    VCons k x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          trans
            (Vec Nat (Suc k))
            (VCons Nat k (add x y) (vector_add k tail_xs tail_ys))
            (VCons Nat k (add y x) (vector_add k tail_xs tail_ys))
            (VCons Nat k (add y x) (vector_add k tail_ys tail_xs))
            (cong
              Nat
              (Vec Nat (Suc k))
              (add x y)
              (add y x)
              (λv. VCons Nat k v (vector_add k tail_xs tail_ys))
              ((proof comm for add) x y))
            (cong
              (Vec Nat k)
              (Vec Nat (Suc k))
              (vector_add k tail_xs tail_ys)
              (vector_add k tail_ys tail_xs)
              (λtail. VCons Nat k (add y x) tail)
              (vector_add_comm k tail_xs tail_ys))
      }
  }

pub theorem vector_add_assoc
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
    : Equal
        (Vec Nat n)
        (vector_add n (vector_add n xs ys) zs)
        (vector_add n xs (vector_add n ys zs)) =
  match xs {
    VNil ↦ Proved;
    VCons k x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          match zs {
            VCons _ z tail_zs ↦
              vector_add_assoc_cons
                k
                x
                y
                z
                tail_xs
                tail_ys
                tail_zs
                (vector_add_assoc k tail_xs tail_ys tail_zs)
          }
      }
  }

pub theorem matrix_vector_additive
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n a (vector_add n x y))
        (vector_add m (matrix_vector m n a x) (matrix_vector m n a y)) =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        old_head = dot n row (vector_add n x y);
        new_head = add (dot n row x) (dot n row y);
        old_tail = matrix_vector k n rest (vector_add n x y);
        new_tail = vector_add k (matrix_vector k n rest x) (matrix_vector k n rest y)
      in
        trans
          (Vec Nat (Suc k))
          (VCons Nat k old_head old_tail)
          (VCons Nat k new_head old_tail)
          (VCons Nat k new_head new_tail)
          (cong
            Nat
            (Vec Nat (Suc k))
            old_head
            new_head
            (λv. VCons Nat k v old_tail)
            (dot_distrib_right n row x y))
          (cong
            (Vec Nat k)
            (Vec Nat (Suc k))
            old_tail
            new_tail
            (VCons Nat k new_head)
            (matrix_vector_additive k n rest x y))
  }

pub theorem matrix_vector_homogeneous
      (m : Nat) (n : Nat) (factor : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n)
    : Equal
        (Vec Nat m)
        (matrix_vector m n a (vector_scale n factor x))
        (vector_scale m factor (matrix_vector m n a x)) =
  match a {
    VNil ↦ Proved;
    VCons k row rest ↦
      let
        old_head = dot n row (vector_scale n factor x);
        new_head = mul factor (dot n row x);
        old_tail = matrix_vector k n rest (vector_scale n factor x);
        new_tail = vector_scale k factor (matrix_vector k n rest x)
      in
        trans
          (Vec Nat (Suc k))
          (VCons Nat k old_head old_tail)
          (VCons Nat k new_head old_tail)
          (VCons Nat k new_head new_tail)
          (cong
            Nat
            (Vec Nat (Suc k))
            old_head
            new_head
            (λv. VCons Nat k v old_tail)
            (dot_scale_right n factor row x))
          (cong
            (Vec Nat k)
            (Vec Nat (Suc k))
            old_tail
            new_tail
            (VCons Nat k new_head)
            (matrix_vector_homogeneous k n factor rest x))
  }

pub theorem transpose_adjoint
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (x : Vec Nat n) (y : Vec Nat m)
    : Equal Nat
        (dot m (matrix_vector m n a x) y)
        (dot n x (matrix_vector n m (transpose m n a) y)) =
  match a {
    VNil ↦
      sym
        Nat
        (dot n x (matrix_vector n Zero (repeat (Vec Nat Zero) n (VNil Nat)) y))
        Zero
        (dot_zero_columns n x y);
    VCons k row tail ↦
      match y {
        VCons _ y0 ys ↦
          let
            row_dot = dot n row x;
            tail_dot = dot k (matrix_vector k n tail x) ys;
            tail_columns = transpose k n tail;
            tail_image = matrix_vector n k tail_columns ys;
            tail_adjoint = dot n x tail_image;
            scaled = vector_scale n y0 row;
            head = mul y0 (dot n x row)
          in
            trans
              Nat
              (add (mul row_dot y0) tail_dot)
              (add head tail_adjoint)
              (dot
                n
                x
                (matrix_vector
                  n
                  (Suc k)
                  (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row tail_columns)
                  (VCons Nat k y0 ys)))
              (trans
                Nat
                (add (mul row_dot y0) tail_dot)
                (add head tail_dot)
                (add head tail_adjoint)
                (cong
                  Nat
                  Nat
                  (mul row_dot y0)
                  head
                  (λv. add v tail_dot)
                  (trans
                    Nat
                    (mul row_dot y0)
                    (mul y0 row_dot)
                    head
                    ((proof comm for mul) row_dot y0)
                    (cong Nat Nat row_dot (dot n x row) (mul y0) (dot_comm n row x))))
                (cong
                  Nat
                  Nat
                  tail_dot
                  tail_adjoint
                  (add head)
                  (transpose_adjoint k n tail x ys)))
              (trans
                Nat
                (add head tail_adjoint)
                (dot n x (vector_add n scaled tail_image))
                (dot
                  n
                  x
                  (matrix_vector
                    n
                    (Suc k)
                    (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row tail_columns)
                    (VCons Nat k y0 ys)))
                (sym
                  Nat
                  (dot n x (vector_add n scaled tail_image))
                  (add head tail_adjoint)
                  (trans
                    Nat
                    (dot n x (vector_add n scaled tail_image))
                    (add (dot n x scaled) tail_adjoint)
                    (add head tail_adjoint)
                    (dot_distrib_right n x scaled tail_image)
                    (cong
                      Nat
                      Nat
                      (dot n x scaled)
                      head
                      (λv. add v tail_adjoint)
                      (dot_scale_right n y0 x row))))
                (cong
                  (Vec Nat n)
                  Nat
                  (vector_add n scaled tail_image)
                  (matrix_vector
                    n
                    (Suc k)
                    (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row tail_columns)
                    (VCons Nat k y0 ys))
                  (dot n x)
                  (sym
                    (Vec Nat n)
                    (matrix_vector
                      n
                      (Suc k)
                      (zip_with
                        Nat
                        (Vec Nat k)
                        (Vec Nat (Suc k))
                        n
                        (prepend k)
                        row
                        tail_columns)
                      (VCons Nat k y0 ys))
                    (vector_add n scaled tail_image)
                    (transpose_cons_image k n row tail_columns y0 ys))))
      }
  }

theorem matrix_add_assoc_cons
      (k : Nat)
      (n : Nat)
      (arow : Vec Nat n)
      (brow : Vec Nat n)
      (crow : Vec Nat n)
      (arest : Vec (Vec Nat n) k)
      (brest : Vec (Vec Nat n) k)
      (crest : Vec (Vec Nat n) k)
      (rest_eq : Equal
        (Vec (Vec Nat n) k)
        (matrix_add k n (matrix_add k n arest brest) crest)
        (matrix_add k n arest (matrix_add k n brest crest)))
    : Equal
        (Vec (Vec Nat n) (Suc k))
        (matrix_add
          (Suc k)
          n
          (matrix_add
            (Suc k)
            n
            (VCons (Vec Nat n) k arow arest)
            (VCons (Vec Nat n) k brow brest))
          (VCons (Vec Nat n) k crow crest))
        (matrix_add
          (Suc k)
          n
          (VCons (Vec Nat n) k arow arest)
          (matrix_add
            (Suc k)
            n
            (VCons (Vec Nat n) k brow brest)
            (VCons (Vec Nat n) k crow crest))) =
  let
    a = VCons (Vec Nat n) k arow arest;
    b = VCons (Vec Nat n) k brow brest;
    c = VCons (Vec Nat n) k crow crest;
    left_rest = matrix_add k n (matrix_add k n arest brest) crest;
    right_rest = matrix_add k n arest (matrix_add k n brest crest);
    left_row = vector_add n (vector_add n arow brow) crow;
    right_row = vector_add n arow (vector_add n brow crow);
    left_inner = VCons (Vec Nat n) k (vector_add n arow brow) (matrix_add k n arest brest);
    right_inner = VCons (Vec Nat n) k (vector_add n brow crow) (matrix_add k n brest crest);
    left_value = VCons (Vec Nat n) k left_row left_rest;
    middle_value = VCons (Vec Nat n) k right_row left_rest;
    right_value = VCons (Vec Nat n) k right_row right_rest
  in
    trans
      (Vec (Vec Nat n) (Suc k))
      (matrix_add (Suc k) n (matrix_add (Suc k) n a b) c)
      (matrix_add (Suc k) n left_inner c)
      (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
      (cong
        (Vec (Vec Nat n) (Suc k))
        (Vec (Vec Nat n) (Suc k))
        (matrix_add (Suc k) n a b)
        left_inner
        (λv. matrix_add (Suc k) n v c)
        (matrix_add_cons k n arow brow arest brest))
      (trans
        (Vec (Vec Nat n) (Suc k))
        (matrix_add (Suc k) n left_inner c)
        left_value
        (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
        (matrix_add_cons k n (vector_add n arow brow) crow (matrix_add k n arest brest) crest)
        (trans
          (Vec (Vec Nat n) (Suc k))
          left_value
          middle_value
          (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
          (cong
            (Vec Nat n)
            (Vec (Vec Nat n) (Suc k))
            left_row
            right_row
            (λrow. VCons (Vec Nat n) k row left_rest)
            (vector_add_assoc n arow brow crow))
          (trans
            (Vec (Vec Nat n) (Suc k))
            middle_value
            right_value
            (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
            (cong
              (Vec (Vec Nat n) k)
              (Vec (Vec Nat n) (Suc k))
              left_rest
              right_rest
              (λrest. VCons (Vec Nat n) k right_row rest)
              rest_eq)
            (sym
              (Vec (Vec Nat n) (Suc k))
              (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
              right_value
              (trans
                (Vec (Vec Nat n) (Suc k))
                (matrix_add (Suc k) n a (matrix_add (Suc k) n b c))
                (matrix_add (Suc k) n a right_inner)
                right_value
                (cong
                  (Vec (Vec Nat n) (Suc k))
                  (Vec (Vec Nat n) (Suc k))
                  (matrix_add (Suc k) n b c)
                  right_inner
                  (λv. matrix_add (Suc k) n a v)
                  (matrix_add_cons k n brow crow brest crest))
                (matrix_add_cons
                  k
                  n
                  arow
                  (vector_add n brow crow)
                  arest
                  (matrix_add k n brest crest)))))))

theorem matrix_add_cons
      (k : Nat)
      (n : Nat)
      (arow : Vec Nat n)
      (brow : Vec Nat n)
      (arest : Vec (Vec Nat n) k)
      (brest : Vec (Vec Nat n) k)
    : Equal
        (Vec (Vec Nat n) (Suc k))
        (matrix_add (Suc k) n (VCons (Vec Nat n) k arow arest) (VCons (Vec Nat n) k brow brest))
        (VCons (Vec Nat n) k (vector_add n arow brow) (matrix_add k n arest brest)) =
  Refl

theorem vector_add_assoc_cons
      (k : Nat)
      (x : Nat)
      (y : Nat)
      (z : Nat)
      (tail_xs : Vec Nat k)
      (tail_ys : Vec Nat k)
      (tail_zs : Vec Nat k)
      (tail_eq : Equal
        (Vec Nat k)
        (vector_add k (vector_add k tail_xs tail_ys) tail_zs)
        (vector_add k tail_xs (vector_add k tail_ys tail_zs)))
    : Equal
        (Vec Nat (Suc k))
        (vector_add
          (Suc k)
          (vector_add (Suc k) (VCons Nat k x tail_xs) (VCons Nat k y tail_ys))
          (VCons Nat k z tail_zs))
        (vector_add
          (Suc k)
          (VCons Nat k x tail_xs)
          (vector_add (Suc k) (VCons Nat k y tail_ys) (VCons Nat k z tail_zs))) =
  let
    xs = VCons Nat k x tail_xs;
    ys = VCons Nat k y tail_ys;
    zs = VCons Nat k z tail_zs;
    left_tail = vector_add k (vector_add k tail_xs tail_ys) tail_zs;
    right_tail = vector_add k tail_xs (vector_add k tail_ys tail_zs);
    left_inner = VCons Nat k (add x y) (vector_add k tail_xs tail_ys);
    right_inner = VCons Nat k (add y z) (vector_add k tail_ys tail_zs);
    left_value = VCons Nat k (add (add x y) z) left_tail;
    middle_value = VCons Nat k (add x (add y z)) left_tail;
    right_value = VCons Nat k (add x (add y z)) right_tail
  in
    trans
      (Vec Nat (Suc k))
      (vector_add (Suc k) (vector_add (Suc k) xs ys) zs)
      (vector_add (Suc k) left_inner zs)
      (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
      (cong
        (Vec Nat (Suc k))
        (Vec Nat (Suc k))
        (vector_add (Suc k) xs ys)
        left_inner
        (λv. vector_add (Suc k) v zs)
        (vector_add_cons k x y tail_xs tail_ys))
      (trans
        (Vec Nat (Suc k))
        (vector_add (Suc k) left_inner zs)
        left_value
        (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
        (vector_add_cons k (add x y) z (vector_add k tail_xs tail_ys) tail_zs)
        (trans
          (Vec Nat (Suc k))
          left_value
          middle_value
          (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
          (cong
            Nat
            (Vec Nat (Suc k))
            (add (add x y) z)
            (add x (add y z))
            (λv. VCons Nat k v left_tail)
            (sym Nat (add x (add y z)) (add (add x y) z) ((proof assoc for add) x y z)))
          (trans
            (Vec Nat (Suc k))
            middle_value
            right_value
            (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
            (cong
              (Vec Nat k)
              (Vec Nat (Suc k))
              left_tail
              right_tail
              (λtail. VCons Nat k (add x (add y z)) tail)
              tail_eq)
            (sym
              (Vec Nat (Suc k))
              (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
              right_value
              (trans
                (Vec Nat (Suc k))
                (vector_add (Suc k) xs (vector_add (Suc k) ys zs))
                (vector_add (Suc k) xs right_inner)
                right_value
                (cong
                  (Vec Nat (Suc k))
                  (Vec Nat (Suc k))
                  (vector_add (Suc k) ys zs)
                  right_inner
                  (λv. vector_add (Suc k) xs v)
                  (vector_add_cons k y z tail_ys tail_zs))
                (vector_add_cons k x (add y z) tail_xs (vector_add k tail_ys tail_zs)))))))

theorem vector_add_cons
      (k : Nat) (x : Nat) (y : Nat) (tail_xs : Vec Nat k) (tail_ys : Vec Nat k)
    : Equal
        (Vec Nat (Suc k))
        (vector_add (Suc k) (VCons Nat k x tail_xs) (VCons Nat k y tail_ys))
        (VCons Nat k (add x y) (vector_add k tail_xs tail_ys)) =
  Refl

fn prepend (n : Nat) (x : Nat) (column : Vec Nat n) : Vec Nat (Suc n) = VCons Nat n x column

fn prepend_zero (n : Nat) (row : Vec Nat n) : Vec Nat (Suc n) = VCons Nat n Zero row

fn repeat (a : Type) (n : Nat) (x : a) : Vec a n =
  match n {
    Zero ↦ VNil a;
    Suc k ↦ VCons a k x (repeat a k x)
  }

fn dot_with (n : Nat) (row : Vec Nat n) (column : Vec Nat n) : Nat = dot n row column

fn row_product_columns
      (n : Nat) (p : Nat) (row : Vec Nat n) (columns : Vec (Vec Nat n) p)
    : Vec Nat p =
  map (Vec Nat n) Nat p (dot_with n row) columns

fn row_product (n : Nat) (p : Nat) (right : Vec (Vec Nat p) n) (row : Vec Nat n) : Vec Nat p =
  row_product_columns n p row (transpose n p right)

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

theorem left_head_empty
      (k : Nat)
    : Equal
        (Vec Nat Zero)
        (row_product_columns
          (Suc k)
          Zero
          (VCons Nat k (Suc Zero) (repeat Nat k Zero))
          (zip_with
            Nat
            (Vec Nat k)
            (Vec Nat (Suc k))
            Zero
            (prepend k)
            (VNil Nat)
            (VNil (Vec Nat k))))
        (VNil Nat) =
  Proved

theorem left_head
      (k : Nat) (n : Nat) (row : Vec Nat n) (columns : Vec (Vec Nat k) n)
    : Equal
        (Vec Nat n)
        (row_product_columns
          (Suc k)
          n
          (VCons Nat k (Suc Zero) (repeat Nat k Zero))
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row columns))
        row =
  match row {
    VNil ↦
      J
        (λcs _.
          Equal
            (Vec Nat Zero)
            (row_product_columns
              (Suc k)
              Zero
              (VCons Nat k (Suc Zero) (repeat Nat k Zero))
              (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) Zero (prepend k) (VNil Nat) cs))
            (VNil Nat))
        (left_head_empty k)
        (vec_zero_vnil (Vec Nat k) columns);
    VCons t x tail_row ↦
      match columns {
        VCons _ col tail_cols ↦
          let
            unit = VCons Nat k (Suc Zero) (repeat Nat k Zero);
            first = dot (Suc k) unit (VCons Nat k x col);
            old_tail =
              row_product_columns
                (Suc k)
                t
                unit
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) t (prepend k) tail_row tail_cols)
          in
            trans
              (Vec Nat (Suc t))
              (VCons Nat t first old_tail)
              (VCons Nat t x old_tail)
              (VCons Nat t x tail_row)
              (cong
                Nat
                (Vec Nat (Suc t))
                first
                x
                (λv. VCons Nat t v old_tail)
                (dot_one_head k x col))
              (cong
                (Vec Nat t)
                (Vec Nat (Suc t))
                old_tail
                tail_row
                (VCons Nat t x)
                (left_head k t tail_row tail_cols))
      }
  }

theorem zero_prefix_columns
      (k : Nat) (p : Nat) (r : Vec Nat k) (row : Vec Nat p) (columns : Vec (Vec Nat k) p)
    : Equal
        (Vec Nat p)
        (row_product_columns
          (Suc k)
          p
          (VCons Nat k Zero r)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) p (prepend k) row columns))
        (row_product_columns k p r columns) =
  match row {
    VNil ↦
      J
        (λcs _.
          Equal
            (Vec Nat Zero)
            (row_product_columns
              (Suc k)
              Zero
              (VCons Nat k Zero r)
              (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) Zero (prepend k) (VNil Nat) cs))
            (row_product_columns k Zero r cs))
        Proved
        (vec_zero_vnil (Vec Nat k) columns);
    VCons t x tail_row ↦
      match columns {
        VCons _ col tail_cols ↦
          let
            prefixed = VCons Nat k Zero r;
            old_head = dot (Suc k) prefixed (VCons Nat k x col);
            new_head = dot k r col;
            old_tail =
              row_product_columns
                (Suc k)
                t
                prefixed
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) t (prepend k) tail_row tail_cols);
            new_tail = row_product_columns k t r tail_cols
          in
            trans
              (Vec Nat (Suc t))
              (VCons Nat t old_head old_tail)
              (VCons Nat t new_head old_tail)
              (VCons Nat t new_head new_tail)
              (cong
                Nat
                (Vec Nat (Suc t))
                old_head
                new_head
                (λv. VCons Nat t v old_tail)
                (dot_zero_prefix k x r col))
              (cong
                (Vec Nat t)
                (Vec Nat (Suc t))
                old_tail
                new_tail
                (VCons Nat t new_head)
                (zero_prefix_columns k t r tail_row tail_cols))
      }
  }

theorem left_tail_rows
      (k : Nat)
      (m : Nat)
      (row : Vec Nat m)
      (rest : Vec (Vec Nat m) k)
      (j : Nat)
      (rows : Vec (Vec Nat k) j)
    : Equal
        (Vec (Vec Nat m) j)
        (map
          (Vec Nat (Suc k))
          (Vec Nat m)
          j
          (row_product (Suc k) m (VCons (Vec Nat m) k row rest))
          (map (Vec Nat k) (Vec Nat (Suc k)) j (prepend_zero k) rows))
        (map (Vec Nat k) (Vec Nat m) j (row_product k m rest) rows) =
  match rows {
    VNil ↦ Proved;
    VCons i r more ↦
      let
        whole = VCons (Vec Nat m) k row rest;
        old_head = row_product (Suc k) m whole (VCons Nat k Zero r);
        new_head = row_product k m rest r;
        old_tail =
          map
            (Vec Nat (Suc k))
            (Vec Nat m)
            i
            (row_product (Suc k) m whole)
            (map (Vec Nat k) (Vec Nat (Suc k)) i (prepend_zero k) more);
        new_tail = map (Vec Nat k) (Vec Nat m) i (row_product k m rest) more
      in
        trans
          (Vec (Vec Nat m) (Suc i))
          (VCons (Vec Nat m) i old_head old_tail)
          (VCons (Vec Nat m) i new_head old_tail)
          (VCons (Vec Nat m) i new_head new_tail)
          (cong
            (Vec Nat m)
            (Vec (Vec Nat m) (Suc i))
            old_head
            new_head
            (λv. VCons (Vec Nat m) i v old_tail)
            (zero_prefix_columns k m r row (transpose k m rest)))
          (cong
            (Vec (Vec Nat m) i)
            (Vec (Vec Nat m) (Suc i))
            old_tail
            new_tail
            (VCons (Vec Nat m) i new_head)
            (left_tail_rows k m row rest i more))
  }

theorem columns_empty_row
      (p : Nat) (columns : Vec (Vec Nat Zero) p)
    : Equal (Vec Nat p) (row_product_columns Zero p (VNil Nat) columns) (repeat Nat p Zero) =
  match columns {
    VNil ↦ Proved;
    VCons j col rest ↦
      cong
        (Vec Nat j)
        (Vec Nat (Suc j))
        (row_product_columns Zero j (VNil Nat) rest)
        (repeat Nat j Zero)
        (VCons Nat j Zero)
        (columns_empty_row j rest)
  }

theorem columns_zero_row
      (n : Nat) (q : Nat) (columns : Vec (Vec Nat n) q)
    : Equal
        (Vec Nat q)
        (row_product_columns n q (repeat Nat n Zero) columns)
        (repeat Nat q Zero) =
  match columns {
    VNil ↦ Proved;
    VCons j col rest ↦
      let
        old_head = dot n (repeat Nat n Zero) col;
        old_tail = row_product_columns n j (repeat Nat n Zero) rest;
        zeros = repeat Nat j Zero
      in
        trans
          (Vec Nat (Suc j))
          (VCons Nat j old_head old_tail)
          (VCons Nat j Zero old_tail)
          (VCons Nat j Zero zeros)
          (cong
            Nat
            (Vec Nat (Suc j))
            old_head
            Zero
            (λv. VCons Nat j v old_tail)
            (dot_zero_left n col))
          (cong
            (Vec Nat j)
            (Vec Nat (Suc j))
            old_tail
            zeros
            (VCons Nat j Zero)
            (columns_zero_row n j rest))
  }

theorem columns_cons
      (k : Nat)
      (p : Nat)
      (x : Nat)
      (rest_row : Vec Nat k)
      (b : Vec Nat p)
      (columns : Vec (Vec Nat k) p)
    : Equal
        (Vec Nat p)
        (row_product_columns
          (Suc k)
          p
          (VCons Nat k x rest_row)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) p (prepend k) b columns))
        (vector_add p (vector_scale p x b) (row_product_columns k p rest_row columns)) =
  match b {
    VNil ↦
      J
        (λcs _.
          Equal
            (Vec Nat Zero)
            (row_product_columns
              (Suc k)
              Zero
              (VCons Nat k x rest_row)
              (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) Zero (prepend k) (VNil Nat) cs))
            (vector_add
              Zero
              (vector_scale Zero x (VNil Nat))
              (row_product_columns k Zero rest_row cs)))
        Proved
        (vec_zero_vnil (Vec Nat k) columns);
    VCons j y tail_b ↦
      match columns {
        VCons _ col tail_cols ↦
          let
            head_value = add (mul x y) (dot k rest_row col);
            old_tail =
              row_product_columns
                (Suc k)
                j
                (VCons Nat k x rest_row)
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) j (prepend k) tail_b tail_cols);
            new_tail =
              vector_add
                j
                (vector_scale j x tail_b)
                (row_product_columns k j rest_row tail_cols)
          in
            cong
              (Vec Nat j)
              (Vec Nat (Suc j))
              old_tail
              new_tail
              (VCons Nat j head_value)
              (columns_cons k j x rest_row tail_b tail_cols)
      }
  }

theorem columns_scale
      (n : Nat) (q : Nat) (x : Nat) (u : Vec Nat n) (columns : Vec (Vec Nat n) q)
    : Equal
        (Vec Nat q)
        (row_product_columns n q (vector_scale n x u) columns)
        (vector_scale q x (row_product_columns n q u columns)) =
  match columns {
    VNil ↦ Proved;
    VCons j col rest ↦
      let
        old_head = dot n (vector_scale n x u) col;
        new_head = mul x (dot n u col);
        old_tail = row_product_columns n j (vector_scale n x u) rest;
        new_tail = vector_scale j x (row_product_columns n j u rest)
      in
        trans
          (Vec Nat (Suc j))
          (VCons Nat j old_head old_tail)
          (VCons Nat j new_head old_tail)
          (VCons Nat j new_head new_tail)
          (cong
            Nat
            (Vec Nat (Suc j))
            old_head
            new_head
            (λv. VCons Nat j v old_tail)
            (dot_scale n x u col))
          (cong
            (Vec Nat j)
            (Vec Nat (Suc j))
            old_tail
            new_tail
            (VCons Nat j new_head)
            (columns_scale n j x u rest))
  }

theorem columns_add
      (n : Nat) (q : Nat) (u : Vec Nat n) (v : Vec Nat n) (columns : Vec (Vec Nat n) q)
    : Equal
        (Vec Nat q)
        (row_product_columns n q (vector_add n u v) columns)
        (vector_add q (row_product_columns n q u columns) (row_product_columns n q v columns)) =
  match columns {
    VNil ↦ Proved;
    VCons j col rest ↦
      let
        old_head = dot n (vector_add n u v) col;
        new_head = add (dot n u col) (dot n v col);
        old_tail = row_product_columns n j (vector_add n u v) rest;
        new_tail =
          vector_add j (row_product_columns n j u rest) (row_product_columns n j v rest)
      in
        trans
          (Vec Nat (Suc j))
          (VCons Nat j old_head old_tail)
          (VCons Nat j new_head old_tail)
          (VCons Nat j new_head new_tail)
          (cong
            Nat
            (Vec Nat (Suc j))
            old_head
            new_head
            (λw. VCons Nat j w old_tail)
            (dot_distrib n u v col))
          (cong
            (Vec Nat j)
            (Vec Nat (Suc j))
            old_tail
            new_tail
            (VCons Nat j new_head)
            (columns_add n j u v rest))
  }

theorem row_product_assoc_empty
      (p : Nat) (q : Nat) (c : Vec (Vec Nat q) p)
    : Equal
        (Vec Nat q)
        (row_product p q c (row_product Zero p (VNil (Vec Nat p)) (VNil Nat)))
        (row_product Zero q (VNil (Vec Nat q)) (VNil Nat)) =
  let
    inner = row_product Zero p (VNil (Vec Nat p)) (VNil Nat);
    zeros_p = repeat Nat p Zero;
    cols = transpose p q c;
    rhs = row_product Zero q (VNil (Vec Nat q)) (VNil Nat)
  in
    trans
      (Vec Nat q)
      (row_product_columns p q inner cols)
      (row_product_columns p q zeros_p cols)
      rhs
      (cong
        (Vec Nat p)
        (Vec Nat q)
        inner
        zeros_p
        (λw. row_product_columns p q w cols)
        (columns_empty_row p (transpose Zero p (VNil (Vec Nat p)))))
      (trans
        (Vec Nat q)
        (row_product_columns p q zeros_p cols)
        (repeat Nat q Zero)
        rhs
        (columns_zero_row p q cols)
        (sym
          (Vec Nat q)
          rhs
          (repeat Nat q Zero)
          (columns_empty_row q (transpose Zero q (VNil (Vec Nat q))))))

theorem row_product_assoc
      (n : Nat)
      (p : Nat)
      (q : Nat)
      (a : Vec Nat n)
      (b : Vec (Vec Nat p) n)
      (c : Vec (Vec Nat q) p)
    : Equal
        (Vec Nat q)
        (row_product p q c (row_product n p b a))
        (row_product n q (matrix_product n p q b c) a) =
  match a {
    VNil ↦
      J
        (λbs _.
          Equal
            (Vec Nat q)
            (row_product p q c (row_product Zero p bs (VNil Nat)))
            (row_product Zero q (matrix_product Zero p q bs c) (VNil Nat)))
        (row_product_assoc_empty p q c)
        (vec_zero_vnil (Vec Nat p) b);
    VCons k x tail_a ↦
      match b {
        VCons _ brow tail_b ↦
          let
            cols = transpose p q c;
            scaled = vector_scale p x brow;
            rest_product = row_product k p tail_b tail_a;
            inner =
              row_product (Suc k) p (VCons (Vec Nat p) k brow tail_b) (VCons Nat k x tail_a);
            split = vector_add p scaled rest_product;
            head_image = row_product_columns p q brow cols;
            rest_bc = matrix_product k p q tail_b c;
            step0 = row_product_columns p q inner cols;
            step1 = row_product_columns p q split cols;
            step2 =
              vector_add
                q
                (row_product_columns p q scaled cols)
                (row_product_columns p q rest_product cols);
            step3 =
              vector_add
                q
                (vector_scale q x head_image)
                (row_product_columns p q rest_product cols);
            step4 = vector_add q (vector_scale q x head_image) (row_product k q rest_bc tail_a);
            goal_rhs =
              row_product
                (Suc k)
                q
                (matrix_product (Suc k) p q (VCons (Vec Nat p) k brow tail_b) c)
                (VCons Nat k x tail_a)
          in
            trans
              (Vec Nat q)
              step0
              step1
              goal_rhs
              (cong
                (Vec Nat p)
                (Vec Nat q)
                inner
                split
                (λw. row_product_columns p q w cols)
                (columns_cons k p x tail_a brow (transpose k p tail_b)))
              (trans
                (Vec Nat q)
                step1
                step2
                goal_rhs
                (columns_add p q scaled rest_product cols)
                (trans
                  (Vec Nat q)
                  step2
                  step3
                  goal_rhs
                  (cong
                    (Vec Nat q)
                    (Vec Nat q)
                    (row_product_columns p q scaled cols)
                    (vector_scale q x head_image)
                    (λw. vector_add q w (row_product_columns p q rest_product cols))
                    (columns_scale p q x brow cols))
                  (trans
                    (Vec Nat q)
                    step3
                    step4
                    goal_rhs
                    (cong
                      (Vec Nat q)
                      (Vec Nat q)
                      (row_product_columns p q rest_product cols)
                      (row_product k q rest_bc tail_a)
                      (vector_add q (vector_scale q x head_image))
                      (row_product_assoc k p q tail_a tail_b c))
                    (sym
                      (Vec Nat q)
                      goal_rhs
                      step4
                      (columns_cons
                        k
                        q
                        x
                        tail_a
                        (row_product p q c brow)
                        (transpose k q rest_bc))))))
      }
  }

theorem dot_row_product_empty
      (p : Nat) (x : Vec Nat p)
    : Equal Nat (dot p (row_product Zero p (VNil (Vec Nat p)) (VNil Nat)) x) Zero =
  let
    inner = row_product Zero p (VNil (Vec Nat p)) (VNil Nat);
    zeros = repeat Nat p Zero
  in
    trans
      Nat
      (dot p inner x)
      (dot p zeros x)
      Zero
      (cong
        (Vec Nat p)
        Nat
        inner
        zeros
        (λw. dot p w x)
        (columns_empty_row p (transpose Zero p (VNil (Vec Nat p)))))
      (dot_zero_left p x)

theorem dot_row_product
      (n : Nat) (p : Nat) (row : Vec Nat n) (b : Vec (Vec Nat p) n) (x : Vec Nat p)
    : Equal Nat (dot p (row_product n p b row) x) (dot n row (matrix_vector n p b x)) =
  match row {
    VNil ↦
      J
        (λbs _.
          Equal
            Nat
            (dot p (row_product Zero p bs (VNil Nat)) x)
            (dot Zero (VNil Nat) (matrix_vector Zero p bs x)))
        (dot_row_product_empty p x)
        (vec_zero_vnil (Vec Nat p) b);
    VCons k y tail_row ↦
      match b {
        VCons _ brow tail_b ↦
          let
            cols = transpose k p tail_b;
            scaled = vector_scale p y brow;
            rest = row_product k p tail_b tail_row;
            inner =
              row_product (Suc k) p (VCons (Vec Nat p) k brow tail_b) (VCons Nat k y tail_row);
            split = vector_add p scaled rest;
            tail_mv = matrix_vector k p tail_b x;
            step0 = dot p inner x;
            step1 = dot p split x;
            step2 = add (dot p scaled x) (dot p rest x);
            step3 = add (mul y (dot p brow x)) (dot p rest x);
            goal = add (mul y (dot p brow x)) (dot k tail_row tail_mv)
          in
            trans
              Nat
              step0
              step1
              goal
              (cong
                (Vec Nat p)
                Nat
                inner
                split
                (λw. dot p w x)
                (columns_cons k p y tail_row brow cols))
              (trans
                Nat
                step1
                step2
                goal
                (dot_distrib p scaled rest x)
                (trans
                  Nat
                  step2
                  step3
                  goal
                  (cong
                    Nat
                    Nat
                    (dot p scaled x)
                    (mul y (dot p brow x))
                    (λv. add v (dot p rest x))
                    (dot_scale p y brow x))
                  (cong
                    Nat
                    Nat
                    (dot p rest x)
                    (dot k tail_row tail_mv)
                    (add (mul y (dot p brow x)))
                    (dot_row_product k p tail_row tail_b x))))
      }
  }

theorem zero_width_repeat
      (p : Nat) (v : Vec (Vec Nat Zero) p)
    : Equal (Vec (Vec Nat Zero) p) (repeat (Vec Nat Zero) p (VNil Nat)) v =
  match v {
    VNil ↦ Proved;
    VCons j col rest ↦
      let zeros =
        repeat (Vec Nat Zero) j (VNil Nat)
      in
        trans
          (Vec (Vec Nat Zero) (Suc j))
          (VCons (Vec Nat Zero) j (VNil Nat) zeros)
          (VCons (Vec Nat Zero) j col zeros)
          (VCons (Vec Nat Zero) j col rest)
          (cong
            (Vec Nat Zero)
            (Vec (Vec Nat Zero) (Suc j))
            (VNil Nat)
            col
            (λh. VCons (Vec Nat Zero) j h zeros)
            (vec_zero_vnil Nat col))
          (cong
            (Vec (Vec Nat Zero) j)
            (Vec (Vec Nat Zero) (Suc j))
            zeros
            rest
            (VCons (Vec Nat Zero) j col)
            (zero_width_repeat j rest))
  }

theorem zip_prepend_map
      (k : Nat)
      (n : Nat)
      (p : Nat)
      (arow : Vec Nat n)
      (at_rest : Vec (Vec Nat k) n)
      (bt : Vec (Vec Nat n) p)
    : Equal
        (Vec (Vec Nat (Suc k)) p)
        (zip_with
          Nat
          (Vec Nat k)
          (Vec Nat (Suc k))
          p
          (prepend k)
          (row_product_columns n p arow bt)
          (map (Vec Nat n) (Vec Nat k) p (row_product n k at_rest) bt))
        (map
          (Vec Nat n)
          (Vec Nat (Suc k))
          p
          (row_product
            n
            (Suc k)
            (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) arow at_rest))
          bt) =
  match bt {
    VNil ↦ Proved;
    VCons j col rest ↦
      let
        zipped = zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) arow at_rest;
        tail_cols = row_product n k at_rest col;
        old_head = VCons Nat k (dot n arow col) tail_cols;
        mid_head = VCons Nat k (dot n col arow) tail_cols;
        new_head = row_product n (Suc k) zipped col;
        old_tail =
          zip_with
            Nat
            (Vec Nat k)
            (Vec Nat (Suc k))
            j
            (prepend k)
            (row_product_columns n j arow rest)
            (map (Vec Nat n) (Vec Nat k) j (row_product n k at_rest) rest);
        new_tail = map (Vec Nat n) (Vec Nat (Suc k)) j (row_product n (Suc k) zipped) rest
      in
        trans
          (Vec (Vec Nat (Suc k)) (Suc j))
          (VCons (Vec Nat (Suc k)) j old_head old_tail)
          (VCons (Vec Nat (Suc k)) j new_head old_tail)
          (VCons (Vec Nat (Suc k)) j new_head new_tail)
          (cong
            (Vec Nat (Suc k))
            (Vec (Vec Nat (Suc k)) (Suc j))
            old_head
            new_head
            (λv. VCons (Vec Nat (Suc k)) j v old_tail)
            (trans
              (Vec Nat (Suc k))
              old_head
              mid_head
              new_head
              (cong
                Nat
                (Vec Nat (Suc k))
                (dot n arow col)
                (dot n col arow)
                (λv. VCons Nat k v tail_cols)
                (dot_comm n arow col))
              (sym
                (Vec Nat (Suc k))
                new_head
                mid_head
                (cong
                  (Vec (Vec Nat n) (Suc k))
                  (Vec Nat (Suc k))
                  (transpose n (Suc k) zipped)
                  (VCons (Vec Nat n) k arow (transpose n k at_rest))
                  (λw. row_product_columns n (Suc k) col w)
                  (transpose_zip_cons k n arow at_rest)))))
          (cong
            (Vec (Vec Nat (Suc k)) j)
            (Vec (Vec Nat (Suc k)) (Suc j))
            old_tail
            new_tail
            (VCons (Vec Nat (Suc k)) j new_head)
            (zip_prepend_map k n j arow at_rest rest))
  }

theorem dot_distrib_right
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
    : Equal Nat (dot n xs (vector_add n ys zs)) (add (dot n xs ys) (dot n xs zs)) =
  trans
    Nat
    (dot n xs (vector_add n ys zs))
    (dot n (vector_add n ys zs) xs)
    (add (dot n xs ys) (dot n xs zs))
    (dot_comm n xs (vector_add n ys zs))
    (trans
      Nat
      (dot n (vector_add n ys zs) xs)
      (add (dot n ys xs) (dot n zs xs))
      (add (dot n xs ys) (dot n xs zs))
      (dot_distrib n ys zs xs)
      (trans
        Nat
        (add (dot n ys xs) (dot n zs xs))
        (add (dot n xs ys) (dot n zs xs))
        (add (dot n xs ys) (dot n xs zs))
        (cong Nat Nat (dot n ys xs) (dot n xs ys) (λv. add v (dot n zs xs)) (dot_comm n ys xs))
        (cong Nat Nat (dot n zs xs) (dot n xs zs) (add (dot n xs ys)) (dot_comm n zs xs))))

theorem dot_scale_right
      (n : Nat) (factor : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal Nat (dot n xs (vector_scale n factor ys)) (mul factor (dot n xs ys)) =
  trans
    Nat
    (dot n xs (vector_scale n factor ys))
    (dot n (vector_scale n factor ys) xs)
    (mul factor (dot n xs ys))
    (dot_comm n xs (vector_scale n factor ys))
    (trans
      Nat
      (dot n (vector_scale n factor ys) xs)
      (mul factor (dot n ys xs))
      (mul factor (dot n xs ys))
      (dot_scale n factor ys xs)
      (cong Nat Nat (dot n ys xs) (dot n xs ys) (mul factor) (dot_comm n ys xs)))

theorem dot_zero_columns
      (n : Nat) (x : Vec Nat n) (y : Vec Nat Zero)
    : Equal Nat (dot n x (matrix_vector n Zero (repeat (Vec Nat Zero) n (VNil Nat)) y)) Zero =
  match x {
    VNil ↦ Proved;
    VCons k x0 tail ↦
      let rest =
        dot k tail (matrix_vector k Zero (repeat (Vec Nat Zero) k (VNil Nat)) y)
      in
        trans
          Nat
          (add (mul x0 Zero) rest)
          rest
          Zero
          ((proof zero_l for add) rest)
          (dot_zero_columns k tail y)
  }

theorem transpose_cons_image
      (k : Nat)
      (n : Nat)
      (row : Vec Nat n)
      (columns : Vec (Vec Nat k) n)
      (y0 : Nat)
      (ys : Vec Nat k)
    : Equal
        (Vec Nat n)
        (matrix_vector
          n
          (Suc k)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) row columns)
          (VCons Nat k y0 ys))
        (vector_add n (vector_scale n y0 row) (matrix_vector n k columns ys)) =
  match row {
    VNil ↦ Proved;
    VCons j r rest ↦
      match columns {
        VCons _ column tail_columns ↦
          let
            column_dot = dot k column ys;
            old_tail =
              matrix_vector
                j
                (Suc k)
                (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) j (prepend k) rest tail_columns)
                (VCons Nat k y0 ys);
            new_tail = vector_add j (vector_scale j y0 rest) (matrix_vector j k tail_columns ys)
          in
            trans
              (Vec Nat (Suc j))
              (VCons Nat j (add (mul r y0) column_dot) old_tail)
              (VCons Nat j (add (mul y0 r) column_dot) old_tail)
              (VCons Nat j (add (mul y0 r) column_dot) new_tail)
              (cong
                Nat
                (Vec Nat (Suc j))
                (mul r y0)
                (mul y0 r)
                (λv. VCons Nat j (add v column_dot) old_tail)
                ((proof comm for mul) r y0))
              (cong
                (Vec Nat j)
                (Vec Nat (Suc j))
                old_tail
                new_tail
                (VCons Nat j (add (mul y0 r) column_dot))
                (transpose_cons_image k j rest tail_columns y0 ys))
      }
  }

theorem columns_add_right
      (n : Nat) (q : Nat) (row : Vec Nat n) (cs : Vec (Vec Nat n) q) (ds : Vec (Vec Nat n) q)
    : Equal
        (Vec Nat q)
        (row_product_columns n q row (matrix_add q n cs ds))
        (vector_add q (row_product_columns n q row cs) (row_product_columns n q row ds)) =
  match cs {
    VNil ↦ Proved;
    VCons j ccol crest ↦
      match ds {
        VCons _ dcol drest ↦
          let
            old_head = dot n row (vector_add n ccol dcol);
            new_head = add (dot n row ccol) (dot n row dcol);
            old_tail = row_product_columns n j row (matrix_add j n crest drest);
            new_tail =
              vector_add
                j
                (row_product_columns n j row crest)
                (row_product_columns n j row drest)
          in
            trans
              (Vec Nat (Suc j))
              (VCons Nat j old_head old_tail)
              (VCons Nat j new_head old_tail)
              (VCons Nat j new_head new_tail)
              (cong
                Nat
                (Vec Nat (Suc j))
                old_head
                new_head
                (λw. VCons Nat j w old_tail)
                (dot_distrib_right n row ccol dcol))
              (cong
                (Vec Nat j)
                (Vec Nat (Suc j))
                old_tail
                new_tail
                (VCons Nat j new_head)
                (columns_add_right n j row crest drest))
      }
  }

theorem row_product_add
      (n : Nat) (p : Nat) (row : Vec Nat n) (b : Vec (Vec Nat p) n) (c : Vec (Vec Nat p) n)
    : Equal
        (Vec Nat p)
        (row_product n p (matrix_add n p b c) row)
        (vector_add p (row_product n p b row) (row_product n p c row)) =
  trans
    (Vec Nat p)
    (row_product_columns n p row (transpose n p (matrix_add n p b c)))
    (row_product_columns n p row (matrix_add p n (transpose n p b) (transpose n p c)))
    (vector_add
      p
      (row_product_columns n p row (transpose n p b))
      (row_product_columns n p row (transpose n p c)))
    (cong
      (Vec (Vec Nat n) p)
      (Vec Nat p)
      (transpose n p (matrix_add n p b c))
      (matrix_add p n (transpose n p b) (transpose n p c))
      (λt. row_product_columns n p row t)
      (transpose_add n p b c))
    (columns_add_right n p row (transpose n p b) (transpose n p c))

theorem zip_prepend_add
      (k : Nat)
      (n : Nat)
      (r1 : Vec Nat n)
      (r2 : Vec Nat n)
      (t1 : Vec (Vec Nat k) n)
      (t2 : Vec (Vec Nat k) n)
    : Equal
        (Vec (Vec Nat (Suc k)) n)
        (zip_with
          Nat
          (Vec Nat k)
          (Vec Nat (Suc k))
          n
          (prepend k)
          (vector_add n r1 r2)
          (matrix_add n k t1 t2))
        (matrix_add
          n
          (Suc k)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) r1 t1)
          (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) n (prepend k) r2 t2)) =
  match r1 {
    VNil ↦ Proved;
    VCons j x1 r1_tail ↦
      match r2 {
        VCons _ x2 r2_tail ↦
          match t1 {
            VCons _ c1 t1_tail ↦
              match t2 {
                VCons _ c2 t2_tail ↦
                  cong
                    (Vec (Vec Nat (Suc k)) j)
                    (Vec (Vec Nat (Suc k)) (Suc j))
                    (zip_with
                      Nat
                      (Vec Nat k)
                      (Vec Nat (Suc k))
                      j
                      (prepend k)
                      (vector_add j r1_tail r2_tail)
                      (matrix_add j k t1_tail t2_tail))
                    (matrix_add
                      j
                      (Suc k)
                      (zip_with Nat (Vec Nat k) (Vec Nat (Suc k)) j (prepend k) r1_tail t1_tail)
                      (zip_with
                        Nat
                        (Vec Nat k)
                        (Vec Nat (Suc k))
                        j
                        (prepend k)
                        r2_tail
                        t2_tail))
                    (VCons (Vec Nat (Suc k)) j (VCons Nat k (add x1 x2) (vector_add k c1 c2)))
                    (zip_prepend_add k j r1_tail r2_tail t1_tail t2_tail)
              }
          }
      }
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
the row proof across the matrix. `matrix_identity_left` instead selects the
head row with the first unit row and removes the zero-prefixed coordinate
from each trailing row. `matrix_product_assoc` distributes column products
over vector sums and scaling. The public `dot_scale` proof uses the checked
natural-number right-distribution law. `matrix_vector_product` distributes a
row's dot product across the right matrix's column products, then lifts the
result across the left matrix by induction on its rows. `transpose_product`
induces on left rows, lifts its tail hypothesis over prepended columns and
commutes each head dot product to reverse the factor order. Its zero-row case
uses the checked empty-width transport. These are generic, indexed proof terms;
the closed example only illustrates their operations.

`matrix_product_distrib_left` lifts a row's sum through transpose and column
products, then recurses over rows. `matrix_product_distrib_right` uses the
existing column-add proof on each pair of summed left rows.
`matrix_vector_distrib` uses the checked dot distributivity law for each row.
`transpose_add` lifts its tail induction through column prepending, with the
empty-width transport handling zero rows. Right-argument dot distribution,
column addition, row-product addition and the prepend bridge stay private.
These four proofs cover arbitrary indexed matrices, including zero dimensions.

`vector_add_comm` commutes each natural sum and recurses on aligned tails;
`matrix_add_comm` lifts this equality across rows. The associativity proofs
rewrite both nested additions through explicit one-level cons equations before
using `cong` and `trans`: a right-nested `zip_with` stays stuck on its inner
result rather than converting to a `VCons` by itself. `vector_add_assoc` reverses
the checked natural associativity proof with `sym` to match the displayed
left-grouped-to-right-grouped direction; `matrix_add_assoc` lifts the vector
step and recursive row equality. These proofs include zero indices.

`matrix_vector_additive` uses the private right-argument dot-distribution
lemma to transport each row's head, then transports the tail by induction.
`matrix_vector_homogeneous` uses a private right-argument scaling lemma
obtained from `dot_comm` and the public `dot_scale` proof. The two results
hold for any row and column counts, including zero.

## 5. Design notes

A matrix is `Vec (Vec Nat n) m`, not a list plus an unchecked width. `transpose`
keeps columns and rows in source order. `identity` places `Suc Zero` on the
diagonal and `Zero` elsewhere. Left identity and product associativity have
distinct generic proofs from right identity. Matrix-vector compatibility and
transposed-product reversal connect the same operations without adding a new
matrix carrier; the row, empty-width and prepend proof stages stay private.
`matrix_add` adds rows with `vector_add`; it neither truncates nor broadcasts,
and its six matrix sum laws hold for arbitrary well-typed dimensions. Two
vector sum laws supply commutativity and associativity for each row. Matrix-
vector application also preserves a sum or scaling in its vector argument;
this differs from adding matrices while keeping the vector fixed. The named
`dot_with` helper passes the row's dot product to `map` without a
lambda-headed application in a proof goal. A rational-valued
or approximate matrix would require a different statement of equality and is
not implied by natural-number arithmetic.

## 6. References

- [Matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
  — Wikipedia; orientation for inner dimensions, ordered products and identity.
- [Vector space](https://en.wikipedia.org/wiki/Vector_space) — Wikipedia;
  background on operations over scalars, not a claim that `Nat` is a field.

## 7. Trust & derivation

The [contract](../../../../spec/50-stdlib/65-linear-algebra.md) §§1–7 owns
twenty-nine public names across eight operations and twenty-one laws. The
[original seed](../../../../conformance/stdlib/linear-algebra/seed-nat-matrix.md),
[distributivity seed](../../../../conformance/stdlib/linear-algebra/seed-matrix-sum-distributivity.md),
[sum-law seed](../../../../conformance/stdlib/linear-algebra/seed-vector-matrix-sum-comm-assoc.md),
[matrix-vector linearity seed](../../../../conformance/stdlib/linear-algebra/seed-matrix-vector-linearity.md)
and [transpose adjoint seed](../../../../conformance/stdlib/linear-algebra/seed-transpose-adjoint.md)
provide finite behavioral clients. The module reuses the published
`Data.Vector.Vector` carrier and natural arithmetic. `repeat`, column
prepending, row products and proof bridges remain private. The new
`dot_zero_columns` and `transpose_cons_image` helpers, like
`dot_scale_right` and `dot_distrib_right`, are not public. All twenty-one
public theorems have checked bodies. This package adds no Axiom, postulate,
primitive, `Float` or trusted-base entry; dependencies retain their own trust
posture. Root-loaded acceptance tests check generic
imports, closed values and indexed relations, isolated refused claims, and
the exact trusted-identity set. The indexed-sum package test checks the authored
positive client and four isolated refusals; the matrix-vector linearity test
checks its authored accepted client and isolated refusal. The transpose
adjoint test checks its complete accepted seed, including both independently
computed dot values, and refuses the swapped-input claim. These tests use
root-loaded exports. They are not official conformance-runner runs or claims
of CI, publication or landing.
