# Natural-coefficient polynomials

Ordered monomials expose the result of distributing natural-coefficient
polynomials. A separate recursive Horner form collects like terms and decides
semantic equality. Substitution replaces variables in a single pass and
preserves evaluation and a positive equality decision under every total
environment. Substitutions also compose in a fixed order and have the variable
constructor as their identity map. Formal differentiation preserves equality
of polynomial functions. These operations have kernel-checked generic laws.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A symbolic sum or product can be evaluated under a total environment without
requiring variable bounds. Distribution into monomials gives an inspectable,
ordered representation of that computation. A coefficient stores the
predecessor of a positive natural number; a zero constant contributes no
monomial. The ordered monomial representation does not sort variables or collect like
terms. A canonical Horner representation supports equality of expressions
that differ in their syntax and distributed monomial lists.

## 2. Definition

`Expr` builds constants, variables, sums and products. `Monomial` pairs a
positive coefficient with an ordered list of variable indices. The public
operations interpret either carrier, distribute products into monomial lists,
reify those lists and normalize by reification. The carriers and their five
constructors are exported alongside the public functions and laws; internal
products and proof bridges remain private. Public laws lead the definition,
so a reader can find each contract before its supporting proofs. The
additional `Poly` carrier stores coefficients in a recursive Horner form;
`canonicalize` builds it from distributed monomials, and `poly_eq` decides
whether two expressions denote the same value under every total environment.
Its three operations and five public laws follow the original definitions in
the same module; the remaining Horner helpers and proof invariants are private.
Substitution is a simultaneous single pass: an inserted expression is not
substituted again. `compose_env` evaluates each variable's replacement under
the original environment, and two laws relate that operation to evaluation
and the forward direction of polynomial equality. These laws do not claim
that substitution preserves a `False` answer. `compose_subst` applies its
second argument first and then its first argument to each inserted expression.
Two checked laws establish structural composition and constructor identity;
private expression congruence proofs support their inductive branches.
`deriv` differentiates each expression constructor structurally. Its two
checked laws say that equal polynomial functions have equal derivatives at
every environment and that `poly_eq` preserves a `True` answer after
differentiation. Private Taylor and natural-arithmetic proofs support them.

```ken
import Core.Logic.Transport (cong, sym, trans)

import Core.Classes.LawfulClasses (IsTrue, bool_and, leq_nat)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l, mul_add_distrib_r)

import Data.Numeric.Nat.Order (sub, leq_nat_add_left_bound, leq_nat_add_right_bound)

import Data.Collections.Derived (list_append)

pub fn normalize (e : Expr) : Expr = reify (monomials e)

pub theorem normalize_sound
      (env : Nat → Nat) (e : Expr)
    : Equal Nat (eval env (normalize e)) (eval env e) =
  trans
    Nat
    (eval env (reify (monomials e)))
    (eval_monomials env (monomials e))
    (eval env e)
    (reify_sound env (monomials e))
    (monomials_sound env e)

pub theorem normalize_idempotent (e : Expr) : Eq Expr (normalize (normalize e)) (normalize e) =
  cong
    (List Monomial)
    Expr
    (monomials (reify (monomials e)))
    (monomials e)
    reify
    (monomials_reify (monomials e))

pub data Expr = EConst Nat | EVar Nat | EAdd Expr Expr | EMul Expr Expr

export EConst, EVar, EAdd, EMul

pub fn eval (env : Nat → Nat) (e : Expr) : Nat =
  match e {
    EConst c ↦ c;
    EVar v ↦ env v;
    EAdd a b ↦ add (eval env a) (eval env b);
    EMul a b ↦ mul (eval env a) (eval env b)
  }

pub data Monomial = Mono Nat (List Nat)

export Mono

pub fn eval_monomial (env : Nat → Nat) (m : Monomial) : Nat =
  match m {
    Mono k vs ↦ mul (Suc k) (var_product env vs)
  }

pub fn eval_monomials (env : Nat → Nat) (ms : List Monomial) : Nat =
  match ms {
    Nil ↦ Zero;
    Cons m rest ↦ add (eval_monomial env m) (eval_monomials env rest)
  }

pub fn monomials (e : Expr) : List Monomial =
  match e {
    EConst c ↦
      match c {
        Zero ↦ Nil Monomial;
        Suc k ↦ Cons Monomial (Mono k (Nil Nat)) (Nil Monomial)
      };
    EVar v ↦ Cons Monomial (Mono Zero (Cons Nat v (Nil Nat))) (Nil Monomial);
    EAdd a b ↦ list_append Monomial (monomials a) (monomials b);
    EMul a b ↦ monomials_product (monomials a) (monomials b)
  }

pub theorem monomials_sound
      (env : Nat → Nat) (e : Expr)
    : Equal Nat (eval_monomials env (monomials e)) (eval env e) =
  match e {
    EConst c ↦
      match c {
        Zero ↦ Proved;
        Suc k ↦ (proof zero_l for add) (Suc k)
      };
    EVar v ↦
      trans
        Nat
        (mul (Suc Zero) (mul (env v) (Suc Zero)))
        (mul (env v) (Suc Zero))
        (env v)
        ((proof one_l for mul) (mul (env v) (Suc Zero)))
        ((proof one_r for mul) (env v));
    EAdd a b ↦
      trans
        Nat
        (eval_monomials env (list_append Monomial (monomials a) (monomials b)))
        (add (eval_monomials env (monomials a)) (eval_monomials env (monomials b)))
        (add (eval env a) (eval env b))
        (eval_monomials_append env (monomials a) (monomials b))
        (trans
          Nat
          (add (eval_monomials env (monomials a)) (eval_monomials env (monomials b)))
          (add (eval env a) (eval_monomials env (monomials b)))
          (add (eval env a) (eval env b))
          (cong
            Nat
            Nat
            (eval_monomials env (monomials a))
            (eval env a)
            (λt. add t (eval_monomials env (monomials b)))
            (monomials_sound env a))
          (cong
            Nat
            Nat
            (eval_monomials env (monomials b))
            (eval env b)
            (λt. add (eval env a) t)
            (monomials_sound env b)));
    EMul a b ↦
      trans
        Nat
        (eval_monomials env (monomials_product (monomials a) (monomials b)))
        (mul (eval_monomials env (monomials a)) (eval_monomials env (monomials b)))
        (mul (eval env a) (eval env b))
        (monomials_product_sound env (monomials a) (monomials b))
        (trans
          Nat
          (mul (eval_monomials env (monomials a)) (eval_monomials env (monomials b)))
          (mul (eval env a) (eval_monomials env (monomials b)))
          (mul (eval env a) (eval env b))
          (cong
            Nat
            Nat
            (eval_monomials env (monomials a))
            (eval env a)
            (λt. mul t (eval_monomials env (monomials b)))
            (monomials_sound env a))
          (cong
            Nat
            Nat
            (eval_monomials env (monomials b))
            (eval env b)
            (λt. mul (eval env a) t)
            (monomials_sound env b)))
  }

pub fn reify (ms : List Monomial) : Expr =
  match ms {
    Nil ↦ EConst Zero;
    Cons m rest ↦
      match m {
        Mono k vs ↦ EAdd (monomial_expr k vs) (reify rest)
      }
  }

pub theorem reify_sound
      (env : Nat → Nat) (ms : List Monomial)
    : Equal Nat (eval env (reify ms)) (eval_monomials env ms) =
  match ms {
    Nil ↦ Proved;
    Cons m rest ↦
      match m {
        Mono k vs ↦
          trans
            Nat
            (add (eval env (monomial_expr k vs)) (eval env (reify rest)))
            (add (mul (Suc k) (var_product env vs)) (eval env (reify rest)))
            (add (mul (Suc k) (var_product env vs)) (eval_monomials env rest))
            (cong
              Nat
              Nat
              (eval env (monomial_expr k vs))
              (mul (Suc k) (var_product env vs))
              (λt. add t (eval env (reify rest)))
              (monomial_expr_sound env k vs))
            (cong
              Nat
              Nat
              (eval env (reify rest))
              (eval_monomials env rest)
              (λt. add (mul (Suc k) (var_product env vs)) t)
              (reify_sound env rest))
      }
  }

pub theorem monomials_reify
      (ms : List Monomial)
    : Eq (List Monomial) (monomials (reify ms)) ms =
  match ms {
    Nil ↦ Proved;
    Cons m rest ↦
      match m {
        Mono k vs ↦
          trans
            (List Monomial)
            (list_append Monomial (monomials (monomial_expr k vs)) (monomials (reify rest)))
            (list_append
              Monomial
              (Cons Monomial (Mono k vs) (Nil Monomial))
              (monomials (reify rest)))
            (Cons Monomial (Mono k vs) rest)
            (cong
              (List Monomial)
              (List Monomial)
              (monomials (monomial_expr k vs))
              (Cons Monomial (Mono k vs) (Nil Monomial))
              (λx. list_append Monomial x (monomials (reify rest)))
              (monomial_expr_roundtrip k vs))
            (cong
              (List Monomial)
              (List Monomial)
              (monomials (reify rest))
              rest
              (λx. Cons Monomial (Mono k vs) x)
              (monomials_reify rest))
      }
  }

fn var_product (env : Nat → Nat) (vs : List Nat) : Nat =
  match vs {
    Nil ↦ Suc Zero;
    Cons v rest ↦ mul (env v) (var_product env rest)
  }

fn coefficient_product (a : Nat) (b : Nat) : Nat = add b (mul (Suc b) a)

fn monomial_product (m : Monomial) (n : Monomial) : Monomial =
  match m {
    Mono a vs ↦
      match n {
        Mono b ws ↦ Mono (coefficient_product a b) (list_append Nat vs ws)
      }
  }

fn scale_monomials (m : Monomial) (ns : List Monomial) : List Monomial =
  match ns {
    Nil ↦ Nil Monomial;
    Cons n rest ↦ Cons Monomial (monomial_product m n) (scale_monomials m rest)
  }

fn monomials_product (ms : List Monomial) (ns : List Monomial) : List Monomial =
  match ms {
    Nil ↦ Nil Monomial;
    Cons m rest ↦ list_append Monomial (scale_monomials m ns) (monomials_product rest ns)
  }

fn monomial_expr (k : Nat) (vs : List Nat) : Expr =
  match vs {
    Nil ↦ EConst (Suc k);
    Cons v rest ↦ EMul (EVar v) (monomial_expr k rest)
  }

theorem monomial_expr_roundtrip
      (k : Nat) (vs : List Nat)
    : Eq
        (List Monomial)
        (monomials (monomial_expr k vs))
        (Cons Monomial (Mono k vs) (Nil Monomial)) =
  match vs {
    Nil ↦ Refl;
    Cons v rest ↦
      cong
        (List Monomial)
        (List Monomial)
        (monomials (monomial_expr k rest))
        (Cons Monomial (Mono k rest) (Nil Monomial))
        (λx.
          monomials_product (Cons Monomial (Mono Zero (Cons Nat v (Nil Nat))) (Nil Monomial)) x)
        (monomial_expr_roundtrip k rest)
  }

theorem mul_swap_left
      (x : Nat) (y : Nat) (z : Nat)
    : Equal Nat (mul x (mul y z)) (mul y (mul x z)) =
  trans
    Nat
    (mul x (mul y z))
    (mul (mul x y) z)
    (mul y (mul x z))
    ((proof assoc for mul) x y z)
    (trans
      Nat
      (mul (mul x y) z)
      (mul (mul y x) z)
      (mul y (mul x z))
      (cong Nat Nat (mul x y) (mul y x) (λt. mul t z) ((proof comm for mul) x y))
      (sym Nat (mul y (mul x z)) (mul (mul y x) z) ((proof assoc for mul) y x z)))

theorem mul_interchange
      (x : Nat) (y : Nat) (z : Nat) (w : Nat)
    : Equal Nat (mul (mul x y) (mul z w)) (mul (mul x z) (mul y w)) =
  trans
    Nat
    (mul (mul x y) (mul z w))
    (mul x (mul y (mul z w)))
    (mul (mul x z) (mul y w))
    (sym
      Nat
      (mul x (mul y (mul z w)))
      (mul (mul x y) (mul z w))
      ((proof assoc for mul) x y (mul z w)))
    (trans
      Nat
      (mul x (mul y (mul z w)))
      (mul x (mul z (mul y w)))
      (mul (mul x z) (mul y w))
      (cong Nat Nat (mul y (mul z w)) (mul z (mul y w)) (λt. mul x t) (mul_swap_left y z w))
      ((proof assoc for mul) x z (mul y w)))

theorem coefficient_product_sound
      (a : Nat) (b : Nat)
    : Equal Nat (Suc (coefficient_product a b)) (mul (Suc a) (Suc b)) =
  cong
    Nat
    Nat
    (add b (mul (Suc b) a))
    (add (mul (Suc a) b) a)
    Suc
    (trans
      Nat
      (add b (mul (Suc b) a))
      (add b (add (mul a b) a))
      (add (mul (Suc a) b) a)
      (cong
        Nat
        Nat
        (mul (Suc b) a)
        (mul a (Suc b))
        (λt. add b t)
        ((proof comm for mul) (Suc b) a))
      (trans
        Nat
        (add b (add (mul a b) a))
        (add (add b (mul a b)) a)
        (add (mul (Suc a) b) a)
        ((proof assoc for add) b (mul a b) a)
        (cong
          Nat
          Nat
          (add b (mul a b))
          (mul (Suc a) b)
          (λt. add t a)
          (trans
            Nat
            (add b (mul a b))
            (add (mul b a) b)
            (mul (Suc a) b)
            (trans
              Nat
              (add b (mul a b))
              (add (mul a b) b)
              (add (mul b a) b)
              ((proof comm for add) b (mul a b))
              (cong Nat Nat (mul a b) (mul b a) (λt. add t b) ((proof comm for mul) a b)))
            ((proof comm for mul) b (Suc a))))))

theorem eval_monomials_append
      (env : Nat → Nat) (p : List Monomial) (q : List Monomial)
    : Equal Nat
        (eval_monomials env (list_append Monomial p q))
        (add (eval_monomials env p) (eval_monomials env q)) =
  match p {
    Nil ↦
      sym
        Nat
        (add Zero (eval_monomials env q))
        (eval_monomials env q)
        ((proof zero_l for add) (eval_monomials env q));
    Cons m rest ↦
      trans
        Nat
        (add (eval_monomial env m) (eval_monomials env (list_append Monomial rest q)))
        (add (eval_monomial env m) (add (eval_monomials env rest) (eval_monomials env q)))
        (add (add (eval_monomial env m) (eval_monomials env rest)) (eval_monomials env q))
        (cong
          Nat
          Nat
          (eval_monomials env (list_append Monomial rest q))
          (add (eval_monomials env rest) (eval_monomials env q))
          (λt. add (eval_monomial env m) t)
          (eval_monomials_append env rest q))
        ((proof assoc for add)
          (eval_monomial env m)
          (eval_monomials env rest)
          (eval_monomials env q))
  }

theorem var_product_append
      (env : Nat → Nat) (vs : List Nat) (ws : List Nat)
    : Equal Nat
        (var_product env (list_append Nat vs ws))
        (mul (var_product env vs) (var_product env ws)) =
  match vs {
    Nil ↦
      sym
        Nat
        (mul (Suc Zero) (var_product env ws))
        (var_product env ws)
        ((proof one_l for mul) (var_product env ws));
    Cons v rest ↦
      trans
        Nat
        (mul (env v) (var_product env (list_append Nat rest ws)))
        (mul (env v) (mul (var_product env rest) (var_product env ws)))
        (mul (mul (env v) (var_product env rest)) (var_product env ws))
        (cong
          Nat
          Nat
          (var_product env (list_append Nat rest ws))
          (mul (var_product env rest) (var_product env ws))
          (λt. mul (env v) t)
          (var_product_append env rest ws))
        ((proof assoc for mul) (env v) (var_product env rest) (var_product env ws))
  }

theorem monomial_product_sound
      (env : Nat → Nat) (m : Monomial) (n : Monomial)
    : Equal Nat
        (eval_monomial env (monomial_product m n))
        (mul (eval_monomial env m) (eval_monomial env n)) =
  match m {
    Mono a vs ↦
      match n {
        Mono b ws ↦
          trans
            Nat
            (mul (Suc (coefficient_product a b)) (var_product env (list_append Nat vs ws)))
            (mul (mul (Suc a) (Suc b)) (var_product env (list_append Nat vs ws)))
            (mul (mul (Suc a) (var_product env vs)) (mul (Suc b) (var_product env ws)))
            (cong
              Nat
              Nat
              (Suc (coefficient_product a b))
              (mul (Suc a) (Suc b))
              (λt. mul t (var_product env (list_append Nat vs ws)))
              (coefficient_product_sound a b))
            (trans
              Nat
              (mul (mul (Suc a) (Suc b)) (var_product env (list_append Nat vs ws)))
              (mul (mul (Suc a) (Suc b)) (mul (var_product env vs) (var_product env ws)))
              (mul (mul (Suc a) (var_product env vs)) (mul (Suc b) (var_product env ws)))
              (cong
                Nat
                Nat
                (var_product env (list_append Nat vs ws))
                (mul (var_product env vs) (var_product env ws))
                (λt. mul (mul (Suc a) (Suc b)) t)
                (var_product_append env vs ws))
              (mul_interchange (Suc a) (Suc b) (var_product env vs) (var_product env ws)))
      }
  }

theorem scale_monomials_sound
      (env : Nat → Nat) (m : Monomial) (ns : List Monomial)
    : Equal Nat
        (eval_monomials env (scale_monomials m ns))
        (mul (eval_monomial env m) (eval_monomials env ns)) =
  match ns {
    Nil ↦ Proved;
    Cons n rest ↦
      trans
        Nat
        (add
          (eval_monomial env (monomial_product m n))
          (eval_monomials env (scale_monomials m rest)))
        (add
          (mul (eval_monomial env m) (eval_monomial env n))
          (mul (eval_monomial env m) (eval_monomials env rest)))
        (mul (eval_monomial env m) (add (eval_monomial env n) (eval_monomials env rest)))
        (trans
          Nat
          (add
            (eval_monomial env (monomial_product m n))
            (eval_monomials env (scale_monomials m rest)))
          (add
            (mul (eval_monomial env m) (eval_monomial env n))
            (eval_monomials env (scale_monomials m rest)))
          (add
            (mul (eval_monomial env m) (eval_monomial env n))
            (mul (eval_monomial env m) (eval_monomials env rest)))
          (cong
            Nat
            Nat
            (eval_monomial env (monomial_product m n))
            (mul (eval_monomial env m) (eval_monomial env n))
            (λt. add t (eval_monomials env (scale_monomials m rest)))
            (monomial_product_sound env m n))
          (cong
            Nat
            Nat
            (eval_monomials env (scale_monomials m rest))
            (mul (eval_monomial env m) (eval_monomials env rest))
            (λt. add (mul (eval_monomial env m) (eval_monomial env n)) t)
            (scale_monomials_sound env m rest)))
        (sym
          Nat
          (mul (eval_monomial env m) (add (eval_monomial env n) (eval_monomials env rest)))
          (add
            (mul (eval_monomial env m) (eval_monomial env n))
            (mul (eval_monomial env m) (eval_monomials env rest)))
          (mul_add_distrib_r
            (eval_monomial env m)
            (eval_monomial env n)
            (eval_monomials env rest)))
  }

theorem monomials_product_sound
      (env : Nat → Nat) (ms : List Monomial) (ns : List Monomial)
    : Equal Nat
        (eval_monomials env (monomials_product ms ns))
        (mul (eval_monomials env ms) (eval_monomials env ns)) =
  match ms {
    Nil ↦
      sym
        Nat
        (mul Zero (eval_monomials env ns))
        Zero
        ((proof zero_l for mul) (eval_monomials env ns));
    Cons m rest ↦
      trans
        Nat
        (eval_monomials
          env
          (list_append Monomial (scale_monomials m ns) (monomials_product rest ns)))
        (add
          (eval_monomials env (scale_monomials m ns))
          (eval_monomials env (monomials_product rest ns)))
        (mul (add (eval_monomial env m) (eval_monomials env rest)) (eval_monomials env ns))
        (eval_monomials_append env (scale_monomials m ns) (monomials_product rest ns))
        (trans
          Nat
          (add
            (eval_monomials env (scale_monomials m ns))
            (eval_monomials env (monomials_product rest ns)))
          (add
            (mul (eval_monomial env m) (eval_monomials env ns))
            (mul (eval_monomials env rest) (eval_monomials env ns)))
          (mul (add (eval_monomial env m) (eval_monomials env rest)) (eval_monomials env ns))
          (trans
            Nat
            (add
              (eval_monomials env (scale_monomials m ns))
              (eval_monomials env (monomials_product rest ns)))
            (add
              (mul (eval_monomial env m) (eval_monomials env ns))
              (eval_monomials env (monomials_product rest ns)))
            (add
              (mul (eval_monomial env m) (eval_monomials env ns))
              (mul (eval_monomials env rest) (eval_monomials env ns)))
            (cong
              Nat
              Nat
              (eval_monomials env (scale_monomials m ns))
              (mul (eval_monomial env m) (eval_monomials env ns))
              (λt. add t (eval_monomials env (monomials_product rest ns)))
              (scale_monomials_sound env m ns))
            (cong
              Nat
              Nat
              (eval_monomials env (monomials_product rest ns))
              (mul (eval_monomials env rest) (eval_monomials env ns))
              (λt. add (mul (eval_monomial env m) (eval_monomials env ns)) t)
              (monomials_product_sound env rest ns)))
          (sym
            Nat
            (mul (add (eval_monomial env m) (eval_monomials env rest)) (eval_monomials env ns))
            (add
              (mul (eval_monomial env m) (eval_monomials env ns))
              (mul (eval_monomials env rest) (eval_monomials env ns)))
            (mul_add_distrib_l
              (eval_monomial env m)
              (eval_monomials env rest)
              (eval_monomials env ns))))
  }

theorem monomial_expr_sound
      (env : Nat → Nat) (k : Nat) (vs : List Nat)
    : Equal Nat (eval env (monomial_expr k vs)) (mul (Suc k) (var_product env vs)) =
  match vs {
    Nil ↦ sym Nat (add Zero (Suc k)) (Suc k) ((proof zero_l for add) (Suc k));
    Cons v rest ↦
      trans
        Nat
        (mul (env v) (eval env (monomial_expr k rest)))
        (mul (env v) (mul (Suc k) (var_product env rest)))
        (mul (Suc k) (mul (env v) (var_product env rest)))
        (cong
          Nat
          Nat
          (eval env (monomial_expr k rest))
          (mul (Suc k) (var_product env rest))
          (λt. mul (env v) t)
          (monomial_expr_sound env k rest))
        (mul_swap_left (env v) (Suc k) (var_product env rest))
  }

pub data Poly = PConst Nat | PHorner Poly Poly

export PConst, PHorner

pub fn eval_poly (env : Nat → Nat) (p : Poly) : Nat =
  match p {
    PConst c ↦ c;
    PHorner p0 q ↦ add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero))
  }

pub fn canonicalize (e : Expr) : Poly = canon (monomials_poly (monomials e))

pub fn poly_eq (e1 : Expr) (e2 : Expr) : Bool = poly_same (canonicalize e1) (canonicalize e2)

pub theorem canonicalize_sound
      (env : Nat → Nat) (e : Expr)
    : Equal Nat (eval_poly env (canonicalize e)) (eval env e) =
  trans
    Nat
    (eval_poly env (canon (monomials_poly (monomials e))))
    (eval_poly env (monomials_poly (monomials e)))
    (eval env e)
    (canon_sound (monomials_poly (monomials e)) env)
    (trans
      Nat
      (eval_poly env (monomials_poly (monomials e)))
      (eval_monomials env (monomials e))
      (eval env e)
      (monomials_poly_sound (monomials e) env)
      (monomials_sound env e))

pub theorem canonicalize_complete
      (e1 : Expr) (e2 : Expr) (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
    : Equal Poly (canonicalize e1) (canonicalize e2) =
  poly_complete
    (canonicalize e1)
    (canonicalize e2)
    (canon_canonical (monomials_poly (monomials e1)))
    (canon_canonical (monomials_poly (monomials e2)))
    (λenv.
      λhe.
        trans
          Nat
          (eval_poly env (canonicalize e1))
          (eval env e1)
          (eval_poly env (canonicalize e2))
          (canonicalize_sound env e1)
          (trans
            Nat
            (eval env e1)
            (eval env e2)
            (eval_poly env (canonicalize e2))
            (h env)
            (sym
              Nat
              (eval_poly env (canonicalize e2))
              (eval env e2)
              (canonicalize_sound env e2))))

pub theorem poly_eq_sound
      (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2)) (env : Nat → Nat)
    : Equal Nat (eval env e1) (eval env e2) =
  trans
    Nat
    (eval env e1)
    (eval_poly env (canonicalize e1))
    (eval env e2)
    (sym Nat (eval_poly env (canonicalize e1)) (eval env e1) (canonicalize_sound env e1))
    (trans
      Nat
      (eval_poly env (canonicalize e1))
      (eval_poly env (canonicalize e2))
      (eval env e2)
      (cong
        Poly
        Nat
        (canonicalize e1)
        (canonicalize e2)
        (λp. eval_poly env p)
        (poly_same_sound (canonicalize e1) (canonicalize e2) h))
      (canonicalize_sound env e2))

pub theorem poly_eq_complete
      (e1 : Expr) (e2 : Expr) (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
    : IsTrue (poly_eq e1 e2) =
  J
    (λy _. IsTrue (poly_same (canonicalize e1) y))
    (poly_same_refl (canonicalize e1))
    (canonicalize_complete e1 e2 h)

pub theorem poly_eq_false_refutes
      (e1 : Expr)
      (e2 : Expr)
      (hf : Equal Bool (poly_eq e1 e2) False)
      (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
    : Bottom =
  trans
    Bool
    False
    (poly_eq e1 e2)
    True
    (sym Bool (poly_eq e1 e2) False hf)
    (poly_eq_complete e1 e2 h)

pub fn compose_env (env : Nat → Nat) (s : Nat → Expr) (v : Nat) : Nat = eval env (s v)

pub fn substitute (s : Nat → Expr) (e : Expr) : Expr =
  match e {
    EConst c ↦ EConst c;
    EVar v ↦ s v;
    EAdd a b ↦ EAdd (substitute s a) (substitute s b);
    EMul a b ↦ EMul (substitute s a) (substitute s b)
  }

pub theorem eval_substitute
      (env : Nat → Nat) (s : Nat → Expr) (e : Expr)
    : Equal Nat (eval env (substitute s e)) (eval (compose_env env s) e) =
  match e {
    EConst c ↦ Refl;
    EVar v ↦ Refl;
    EAdd a b ↦
      add_cong
        (eval env (substitute s a))
        (eval (compose_env env s) a)
        (eval env (substitute s b))
        (eval (compose_env env s) b)
        (eval_substitute env s a)
        (eval_substitute env s b);
    EMul a b ↦
      mul_cong
        (eval env (substitute s a))
        (eval (compose_env env s) a)
        (eval env (substitute s b))
        (eval (compose_env env s) b)
        (eval_substitute env s a)
        (eval_substitute env s b)
  }

pub theorem poly_eq_substitute
      (s : Nat → Expr) (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2))
    : IsTrue (poly_eq (substitute s e1) (substitute s e2)) =
  poly_eq_complete
    (substitute s e1)
    (substitute s e2)
    (λenv.
      let composed : Nat → Nat =
        compose_env env s
      in
        trans
          Nat
          (eval env (substitute s e1))
          (eval composed e1)
          (eval env (substitute s e2))
          (eval_substitute env s e1)
          (trans
            Nat
            (eval composed e1)
            (eval composed e2)
            (eval env (substitute s e2))
            (poly_eq_sound e1 e2 h composed)
            (sym
              Nat
              (eval env (substitute s e2))
              (eval composed e2)
              (eval_substitute env s e2))))

pub fn compose_subst (s : Nat → Expr) (t : Nat → Expr) (v : Nat) : Expr = substitute s (t v)

pub theorem substitute_compose
      (s : Nat → Expr) (t : Nat → Expr) (e : Expr)
    : Equal Expr (substitute s (substitute t e)) (substitute (compose_subst s t) e) =
  match e {
    EConst c ↦ Refl;
    EVar v ↦ Refl;
    EAdd a b ↦
      expr_add_cong
        (substitute s (substitute t a))
        (substitute (compose_subst s t) a)
        (substitute s (substitute t b))
        (substitute (compose_subst s t) b)
        (substitute_compose s t a)
        (substitute_compose s t b);
    EMul a b ↦
      expr_mul_cong
        (substitute s (substitute t a))
        (substitute (compose_subst s t) a)
        (substitute s (substitute t b))
        (substitute (compose_subst s t) b)
        (substitute_compose s t a)
        (substitute_compose s t b)
  }

pub theorem substitute_var (e : Expr) : Equal Expr (substitute EVar e) e =
  match e {
    EConst c ↦ Refl;
    EVar v ↦ Refl;
    EAdd a b ↦
      expr_add_cong
        (substitute EVar a)
        a
        (substitute EVar b)
        b
        (substitute_var a)
        (substitute_var b);
    EMul a b ↦
      expr_mul_cong
        (substitute EVar a)
        a
        (substitute EVar b)
        b
        (substitute_var a)
        (substitute_var b)
  }

pub fn deriv (v : Nat) (e : Expr) : Expr =
  match e {
    EConst c ↦ EConst Zero;
    EVar w ↦ var_deriv (nat_same v w);
    EAdd a b ↦ EAdd (deriv v a) (deriv v b);
    EMul a b ↦ EAdd (EMul (deriv v a) b) (EMul a (deriv v b))
  }

pub theorem deriv_respects_eval
      (v : Nat)
      (e1 : Expr)
      (e2 : Expr)
      (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
      (env : Nat → Nat)
    : Equal Nat (eval env (deriv v e1)) (eval env (deriv v e2)) =
  let
    d1 : Nat = eval env (deriv v e1);
    d2 : Nat = eval env (deriv v e2);
    t : Nat = Suc (add d1 d2);
    r1 : Nat = taylor_rem v t env e1;
    r2 : Nat = taylor_rem v t env e2;
    bumped : Nat → Nat = bump_env v t env
  in
    deriv_extract
      (eval env e1)
      (eval env e2)
      t
      d1
      d2
      r1
      r2
      Proved
      (leq_nat_add_left_bound d1 d2)
      (leq_nat_add_right_bound d1 d2)
      (h env)
      (trans
        Nat
        (add (eval env e1) (mul t (add d1 (mul t r1))))
        (eval bumped e1)
        (add (eval env e2) (mul t (add d2 (mul t r2))))
        (sym
          Nat
          (eval bumped e1)
          (add (eval env e1) (mul t (add d1 (mul t r1))))
          (deriv_taylor v t env e1))
        (trans
          Nat
          (eval bumped e1)
          (eval bumped e2)
          (add (eval env e2) (mul t (add d2 (mul t r2))))
          (h bumped)
          (deriv_taylor v t env e2)))

pub theorem poly_eq_deriv
      (v : Nat) (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2))
    : IsTrue (poly_eq (deriv v e1) (deriv v e2)) =
  poly_eq_complete
    (deriv v e1)
    (deriv v e2)
    (λenv. deriv_respects_eval v e1 e2 (λg. poly_eq_sound e1 e2 h g) env)

theorem expr_add_cong
      (a1 : Expr)
      (a2 : Expr)
      (b1 : Expr)
      (b2 : Expr)
      (p : Equal Expr a1 a2)
      (q : Equal Expr b1 b2)
    : Equal Expr (EAdd a1 b1) (EAdd a2 b2) =
  trans
    Expr
    (EAdd a1 b1)
    (EAdd a2 b1)
    (EAdd a2 b2)
    (cong Expr Expr a1 a2 (λx. EAdd x b1) p)
    (cong Expr Expr b1 b2 (λx. EAdd a2 x) q)

theorem expr_mul_cong
      (a1 : Expr)
      (a2 : Expr)
      (b1 : Expr)
      (b2 : Expr)
      (p : Equal Expr a1 a2)
      (q : Equal Expr b1 b2)
    : Equal Expr (EMul a1 b1) (EMul a2 b2) =
  trans
    Expr
    (EMul a1 b1)
    (EMul a2 b1)
    (EMul a2 b2)
    (cong Expr Expr a1 a2 (λx. EMul x b1) p)
    (cong Expr Expr b1 b2 (λx. EMul a2 x) q)

fn shift_env (env : Nat → Nat) (v : Nat) : Nat = env (Suc v)

fn add_const (c : Nat) (p : Poly) : Poly =
  match p {
    PConst d ↦ PConst (add c d);
    PHorner p0 q ↦ PHorner (add_const c p0) q
  }

fn add_poly (p : Poly) (r : Poly) : Poly =
  match p {
    PConst c ↦ add_const c r;
    PHorner p0 q ↦
      match r {
        PConst d ↦ PHorner (add_const d p0) q;
        PHorner r0 s ↦ PHorner (add_poly p0 r0) (add_poly q s)
      }
  }

fn var_poly (v : Nat) (c : Nat) : Poly =
  match v {
    Zero ↦ PHorner (PConst Zero) (PConst c);
    Suc w ↦ PHorner (var_poly w c) (PConst Zero)
  }

fn mul_var (p : Poly) (v : Nat) : Poly =
  match p {
    PConst c ↦ var_poly v c;
    PHorner p0 q ↦
      match v {
        Zero ↦ PHorner (PConst Zero) (PHorner p0 q);
        Suc w ↦ PHorner (mul_var p0 w) (mul_var q (Suc w))
      }
  }

fn vars_poly (vs : List Nat) (k : Nat) : Poly =
  match vs {
    Nil ↦ PConst (Suc k);
    Cons v rest ↦ mul_var (vars_poly rest k) v
  }

fn monomials_poly (ms : List Monomial) : Poly =
  match ms {
    Nil ↦ PConst Zero;
    Cons m rest ↦
      match m {
        Mono k vs ↦ add_poly (vars_poly vs k) (monomials_poly rest)
      }
  }

fn mk_horner (p : Poly) (q : Poly) : Poly =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          match p {
            PConst d ↦ PConst d;
            PHorner p1 q1 ↦ PHorner (PHorner p1 q1) (PConst Zero)
          };
        Suc c1 ↦ PHorner p (PConst (Suc c1))
      };
    PHorner q1 q2 ↦ PHorner p (PHorner q1 q2)
  }

fn canon (p : Poly) : Poly =
  match p {
    PConst c ↦ PConst c;
    PHorner p0 q ↦ mk_horner (canon p0) (canon q)
  }

fn nondegenerate (p : Poly) (q : Poly) : Bool =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          match p {
            PConst d ↦ False;
            PHorner p1 q1 ↦ True
          };
        Suc c1 ↦ True
      };
    PHorner q1 q2 ↦ True
  }

fn is_canonical (p : Poly) : Bool =
  match p {
    PConst c ↦ True;
    PHorner p0 q ↦ bool_and (nondegenerate p0 q) (bool_and (is_canonical p0) (is_canonical q))
  }

fn is_zero_poly (p : Poly) : Bool =
  match p {
    PConst c ↦
      match c {
        Zero ↦ True;
        Suc c1 ↦ False
      };
    PHorner p0 q ↦ False
  }

fn nat_same (a : Nat) (b : Nat) : Bool = bool_and (leq_nat a b) (leq_nat b a)

fn poly_same (p : Poly) (r : Poly) : Bool =
  match p {
    PConst c ↦
      match r {
        PConst d ↦ nat_same c d;
        PHorner r0 s ↦ False
      };
    PHorner p0 q ↦
      match r {
        PConst d ↦ False;
        PHorner r0 s ↦ bool_and (poly_same p0 r0) (poly_same q s)
      }
  }

fn cons_env (x : Nat) (g : Nat → Nat) (v : Nat) : Nat =
  match v {
    Zero ↦ x;
    Suc w ↦ g w
  }

fn one_env (v : Nat) : Nat = Suc Zero

theorem add_cong
      (a1 : Nat) (a2 : Nat) (b1 : Nat) (b2 : Nat) (p : Equal Nat a1 a2) (q : Equal Nat b1 b2)
    : Equal Nat (add a1 b1) (add a2 b2) =
  trans
    Nat
    (add a1 b1)
    (add a2 b1)
    (add a2 b2)
    (cong Nat Nat a1 a2 (λt. add t b1) p)
    (cong Nat Nat b1 b2 (λt. add a2 t) q)

theorem mul_cong
      (a1 : Nat) (a2 : Nat) (b1 : Nat) (b2 : Nat) (p : Equal Nat a1 a2) (q : Equal Nat b1 b2)
    : Equal Nat (mul a1 b1) (mul a2 b2) =
  trans
    Nat
    (mul a1 b1)
    (mul a2 b1)
    (mul a2 b2)
    (cong Nat Nat a1 a2 (λt. mul t b1) p)
    (cong Nat Nat b1 b2 (λt. mul a2 t) q)

theorem add_interchange
      (a : Nat) (b : Nat) (c : Nat) (d : Nat)
    : Equal Nat (add (add a b) (add c d)) (add (add a c) (add b d)) =
  trans
    Nat
    (add (add a b) (add c d))
    (add a (add b (add c d)))
    (add (add a c) (add b d))
    (sym
      Nat
      (add a (add b (add c d)))
      (add (add a b) (add c d))
      ((proof assoc for add) a b (add c d)))
    (trans
      Nat
      (add a (add b (add c d)))
      (add a (add c (add b d)))
      (add (add a c) (add b d))
      (cong
        Nat
        Nat
        (add b (add c d))
        (add c (add b d))
        (λt. add a t)
        (trans
          Nat
          (add b (add c d))
          (add (add b c) d)
          (add c (add b d))
          ((proof assoc for add) b c d)
          (trans
            Nat
            (add (add b c) d)
            (add (add c b) d)
            (add c (add b d))
            (cong Nat Nat (add b c) (add c b) (λt. add t d) ((proof comm for add) b c))
            (sym Nat (add c (add b d)) (add (add c b) d) ((proof assoc for add) c b d)))))
      ((proof assoc for add) a c (add b d)))

theorem add_swap_tail
      (d : Nat) (a : Nat) (m : Nat)
    : Equal Nat (add (add d a) m) (add (add a m) d) =
  trans
    Nat
    (add (add d a) m)
    (add (add a d) m)
    (add (add a m) d)
    (cong Nat Nat (add d a) (add a d) (λt. add t m) ((proof comm for add) d a))
    (trans
      Nat
      (add (add a d) m)
      (add a (add d m))
      (add (add a m) d)
      (sym Nat (add a (add d m)) (add (add a d) m) ((proof assoc for add) a d m))
      (trans
        Nat
        (add a (add d m))
        (add a (add m d))
        (add (add a m) d)
        (cong Nat Nat (add d m) (add m d) (λt. add a t) ((proof comm for add) d m))
        ((proof assoc for add) a m d)))

theorem mul_swap_right
      (q : Nat) (y : Nat) (x : Nat)
    : Equal Nat (mul (mul q y) x) (mul (mul q x) y) =
  trans
    Nat
    (mul (mul q y) x)
    (mul q (mul y x))
    (mul (mul q x) y)
    (sym Nat (mul q (mul y x)) (mul (mul q y) x) ((proof assoc for mul) q y x))
    (trans
      Nat
      (mul q (mul y x))
      (mul q (mul x y))
      (mul (mul q x) y)
      (cong Nat Nat (mul y x) (mul x y) (λt. mul q t) ((proof comm for mul) y x))
      ((proof assoc for mul) q x y))

theorem mul_rotate
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (mul a b) c) (mul a (mul c b)) =
  trans
    Nat
    (mul (mul a b) c)
    (mul a (mul b c))
    (mul a (mul c b))
    (sym Nat (mul a (mul b c)) (mul (mul a b) c) ((proof assoc for mul) a b c))
    (cong Nat Nat (mul b c) (mul c b) (λt. mul a t) ((proof comm for mul) b c))

theorem add_const_sound
      (p : Poly)
    : (env : Nat → Nat)
      → (c : Nat)
      → Equal Nat (eval_poly env (add_const c p)) (add c (eval_poly env p)) =
  match p {
    PConst d ↦ λenv. λc. Refl;
    PHorner p0 q ↦
      λenv.
        λc.
          trans
            Nat
            (add
              (eval_poly (shift_env env) (add_const c p0))
              (mul (eval_poly env q) (env Zero)))
            (add (add c (eval_poly (shift_env env) p0)) (mul (eval_poly env q) (env Zero)))
            (add c (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero))))
            (cong
              Nat
              Nat
              (eval_poly (shift_env env) (add_const c p0))
              (add c (eval_poly (shift_env env) p0))
              (λt. add t (mul (eval_poly env q) (env Zero)))
              (add_const_sound p0 (shift_env env) c))
            (sym
              Nat
              (add c (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero))))
              (add (add c (eval_poly (shift_env env) p0)) (mul (eval_poly env q) (env Zero)))
              ((proof assoc for add)
                c
                (eval_poly (shift_env env) p0)
                (mul (eval_poly env q) (env Zero))))
  }

theorem add_poly_sound
      (p : Poly)
    : (r : Poly)
      → (env : Nat → Nat)
      → Equal Nat (eval_poly env (add_poly p r)) (add (eval_poly env p) (eval_poly env r)) =
  match p {
    PConst c ↦ λr. λenv. add_const_sound r env c;
    PHorner p0 q ↦
      λr.
        match r {
          PConst d ↦
            λenv.
              trans
                Nat
                (add
                  (eval_poly (shift_env env) (add_const d p0))
                  (mul (eval_poly env q) (env Zero)))
                (add (add d (eval_poly (shift_env env) p0)) (mul (eval_poly env q) (env Zero)))
                (add (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero))) d)
                (cong
                  Nat
                  Nat
                  (eval_poly (shift_env env) (add_const d p0))
                  (add d (eval_poly (shift_env env) p0))
                  (λt. add t (mul (eval_poly env q) (env Zero)))
                  (add_const_sound p0 (shift_env env) d))
                (add_swap_tail
                  d
                  (eval_poly (shift_env env) p0)
                  (mul (eval_poly env q) (env Zero)));
          PHorner r0 s ↦
            λenv.
              trans
                Nat
                (add
                  (eval_poly (shift_env env) (add_poly p0 r0))
                  (mul (eval_poly env (add_poly q s)) (env Zero)))
                (add
                  (add (eval_poly (shift_env env) p0) (eval_poly (shift_env env) r0))
                  (add (mul (eval_poly env q) (env Zero)) (mul (eval_poly env s) (env Zero))))
                (add
                  (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero)))
                  (add (eval_poly (shift_env env) r0) (mul (eval_poly env s) (env Zero))))
                (add_cong
                  (eval_poly (shift_env env) (add_poly p0 r0))
                  (add (eval_poly (shift_env env) p0) (eval_poly (shift_env env) r0))
                  (mul (eval_poly env (add_poly q s)) (env Zero))
                  (add (mul (eval_poly env q) (env Zero)) (mul (eval_poly env s) (env Zero)))
                  (add_poly_sound p0 r0 (shift_env env))
                  (trans
                    Nat
                    (mul (eval_poly env (add_poly q s)) (env Zero))
                    (mul (add (eval_poly env q) (eval_poly env s)) (env Zero))
                    (add (mul (eval_poly env q) (env Zero)) (mul (eval_poly env s) (env Zero)))
                    (cong
                      Nat
                      Nat
                      (eval_poly env (add_poly q s))
                      (add (eval_poly env q) (eval_poly env s))
                      (λt. mul t (env Zero))
                      (add_poly_sound q s env))
                    (mul_add_distrib_l (eval_poly env q) (eval_poly env s) (env Zero))))
                (add_interchange
                  (eval_poly (shift_env env) p0)
                  (eval_poly (shift_env env) r0)
                  (mul (eval_poly env q) (env Zero))
                  (mul (eval_poly env s) (env Zero)))
        }
  }

theorem var_poly_sound
      (v : Nat)
    : (env : Nat → Nat) → (c : Nat) → Equal Nat (eval_poly env (var_poly v c)) (mul c (env v)) =
  match v {
    Zero ↦ λenv. λc. (proof zero_l for add) (mul c (env Zero));
    Suc w ↦
      λenv.
        λc.
          add_cong
            (eval_poly (shift_env env) (var_poly w c))
            (mul c (env (Suc w)))
            (mul Zero (env Zero))
            Zero
            (var_poly_sound w (shift_env env) c)
            ((proof zero_l for mul) (env Zero))
  }

theorem mul_var_sound
      (p : Poly)
    : (v : Nat)
      → (env : Nat → Nat)
      → Equal Nat (eval_poly env (mul_var p v)) (mul (eval_poly env p) (env v)) =
  match p {
    PConst c ↦ λv. λenv. var_poly_sound v env c;
    PHorner p0 q ↦
      λv.
        match v {
          Zero ↦ λenv. (proof zero_l for add) (mul (eval_poly env (PHorner p0 q)) (env Zero));
          Suc w ↦
            λenv.
              trans
                Nat
                (add
                  (eval_poly (shift_env env) (mul_var p0 w))
                  (mul (eval_poly env (mul_var q (Suc w))) (env Zero)))
                (add
                  (mul (eval_poly (shift_env env) p0) (env (Suc w)))
                  (mul (mul (eval_poly env q) (env (Suc w))) (env Zero)))
                (mul
                  (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero)))
                  (env (Suc w)))
                (add_cong
                  (eval_poly (shift_env env) (mul_var p0 w))
                  (mul (eval_poly (shift_env env) p0) (env (Suc w)))
                  (mul (eval_poly env (mul_var q (Suc w))) (env Zero))
                  (mul (mul (eval_poly env q) (env (Suc w))) (env Zero))
                  (mul_var_sound p0 w (shift_env env))
                  (cong
                    Nat
                    Nat
                    (eval_poly env (mul_var q (Suc w)))
                    (mul (eval_poly env q) (env (Suc w)))
                    (λt. mul t (env Zero))
                    (mul_var_sound q (Suc w) env)))
                (trans
                  Nat
                  (add
                    (mul (eval_poly (shift_env env) p0) (env (Suc w)))
                    (mul (mul (eval_poly env q) (env (Suc w))) (env Zero)))
                  (add
                    (mul (eval_poly (shift_env env) p0) (env (Suc w)))
                    (mul (mul (eval_poly env q) (env Zero)) (env (Suc w))))
                  (mul
                    (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero)))
                    (env (Suc w)))
                  (cong
                    Nat
                    Nat
                    (mul (mul (eval_poly env q) (env (Suc w))) (env Zero))
                    (mul (mul (eval_poly env q) (env Zero)) (env (Suc w)))
                    (λt. add (mul (eval_poly (shift_env env) p0) (env (Suc w))) t)
                    (mul_swap_right (eval_poly env q) (env (Suc w)) (env Zero)))
                  (sym
                    Nat
                    (mul
                      (add (eval_poly (shift_env env) p0) (mul (eval_poly env q) (env Zero)))
                      (env (Suc w)))
                    (add
                      (mul (eval_poly (shift_env env) p0) (env (Suc w)))
                      (mul (mul (eval_poly env q) (env Zero)) (env (Suc w))))
                    (mul_add_distrib_l
                      (eval_poly (shift_env env) p0)
                      (mul (eval_poly env q) (env Zero))
                      (env (Suc w)))))
        }
  }

theorem vars_poly_sound
      (vs : List Nat)
    : (k : Nat)
      → (env : Nat → Nat)
      → Equal Nat (eval_poly env (vars_poly vs k)) (eval_monomial env (Mono k vs)) =
  match vs {
    Nil ↦ λk. λenv. sym Nat (mul (Suc k) (Suc Zero)) (Suc k) ((proof one_r for mul) (Suc k));
    Cons v rest ↦
      λk.
        λenv.
          trans
            Nat
            (eval_poly env (mul_var (vars_poly rest k) v))
            (mul (eval_poly env (vars_poly rest k)) (env v))
            (mul (Suc k) (mul (env v) (var_product env rest)))
            (mul_var_sound (vars_poly rest k) v env)
            (trans
              Nat
              (mul (eval_poly env (vars_poly rest k)) (env v))
              (mul (mul (Suc k) (var_product env rest)) (env v))
              (mul (Suc k) (mul (env v) (var_product env rest)))
              (cong
                Nat
                Nat
                (eval_poly env (vars_poly rest k))
                (mul (Suc k) (var_product env rest))
                (λt. mul t (env v))
                (vars_poly_sound rest k env))
              (mul_rotate (Suc k) (var_product env rest) (env v)))
  }

theorem monomials_poly_sound
      (ms : List Monomial)
    : (env : Nat → Nat)
      → Equal Nat (eval_poly env (monomials_poly ms)) (eval_monomials env ms) =
  match ms {
    Nil ↦ λenv. Proved;
    Cons m rest ↦
      match m {
        Mono k vs ↦
          λenv.
            trans
              Nat
              (eval_poly env (add_poly (vars_poly vs k) (monomials_poly rest)))
              (add (eval_poly env (vars_poly vs k)) (eval_poly env (monomials_poly rest)))
              (add (eval_monomial env (Mono k vs)) (eval_monomials env rest))
              (add_poly_sound (vars_poly vs k) (monomials_poly rest) env)
              (add_cong
                (eval_poly env (vars_poly vs k))
                (eval_monomial env (Mono k vs))
                (eval_poly env (monomials_poly rest))
                (eval_monomials env rest)
                (vars_poly_sound vs k env)
                (monomials_poly_sound rest env))
      }
  }

theorem mk_horner_sound
      (q : Poly)
    : (p : Poly)
      → (env : Nat → Nat)
      → Equal Nat (eval_poly env (mk_horner p q)) (eval_poly env (PHorner p q)) =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          λp.
            match p {
              PConst d ↦
                λenv.
                  sym
                    Nat
                    (add d (mul Zero (env Zero)))
                    d
                    (cong
                      Nat
                      Nat
                      (mul Zero (env Zero))
                      Zero
                      (λt. add d t)
                      ((proof zero_l for mul) (env Zero)));
              PHorner p1 q1 ↦ λenv. Refl
            };
        Suc c1 ↦ λp. λenv. Refl
      };
    PHorner q1 q2 ↦ λp. λenv. Refl
  }

theorem canon_sound
      (p : Poly)
    : (env : Nat → Nat) → Equal Nat (eval_poly env (canon p)) (eval_poly env p) =
  match p {
    PConst c ↦ λenv. Refl;
    PHorner p0 q ↦
      λenv.
        trans
          Nat
          (eval_poly env (mk_horner (canon p0) (canon q)))
          (eval_poly env (PHorner (canon p0) (canon q)))
          (eval_poly env (PHorner p0 q))
          (mk_horner_sound (canon q) (canon p0) env)
          (add_cong
            (eval_poly (shift_env env) (canon p0))
            (eval_poly (shift_env env) p0)
            (mul (eval_poly env (canon q)) (env Zero))
            (mul (eval_poly env q) (env Zero))
            (canon_sound p0 (shift_env env))
            (cong
              Nat
              Nat
              (eval_poly env (canon q))
              (eval_poly env q)
              (λt. mul t (env Zero))
              (canon_sound q env)))
  }

theorem mk_horner_canonical
      (q : Poly)
    : (p : Poly)
      → IsTrue (is_canonical p)
      → IsTrue (is_canonical q)
      → IsTrue (is_canonical (mk_horner p q)) =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          λp.
            match p {
              PConst d ↦ λhp. λhq. Proved;
              PHorner p1 q1 ↦
                λhp.
                  λhq. (proof intro for bool_and) (is_canonical (PHorner p1 q1)) True hp Proved
            };
        Suc c1 ↦ λp. λhp. λhq. (proof intro for bool_and) (is_canonical p) True hp Proved
      };
    PHorner q1 q2 ↦
      λp.
        λhp.
          λhq. (proof intro for bool_and) (is_canonical p) (is_canonical (PHorner q1 q2)) hp hq
  }

theorem canon_canonical (p : Poly) : IsTrue (is_canonical (canon p)) =
  match p {
    PConst c ↦ Proved;
    PHorner p0 q ↦
      mk_horner_canonical (canon q) (canon p0) (canon_canonical p0) (canon_canonical q)
  }

theorem suc_leq_self (a : Nat) : IsTrue (leq_nat (Suc a) a) → Bottom =
  match a {
    Zero ↦ λh. h;
    Suc a2 ↦ λh. suc_leq_self a2 h
  }

theorem mul_positive
      (a : Nat) (b : Nat)
    : IsTrue (leq_nat (Suc Zero) a)
      → IsTrue (leq_nat (Suc Zero) b)
      → IsTrue (leq_nat (Suc Zero) (mul a b)) =
  match b {
    Zero ↦ λha. λhb. absurd hb;
    Suc b2 ↦
      λha.
        λhb.
          (proof trans for leq_nat)
            (Suc Zero)
            a
            (add (mul a b2) a)
            ha
            (leq_nat_add_right_bound (mul a b2) a)
  }

theorem mul_ge_right
      (q : Nat) (k : Nat)
    : IsTrue (leq_nat (Suc Zero) q) → IsTrue (leq_nat k (mul q k)) =
  match q {
    Zero ↦ λh. absurd h;
    Suc q2 ↦
      λh.
        J
          (λy _. IsTrue (leq_nat k y))
          (leq_nat_add_right_bound (mul k q2) k)
          ((proof comm for mul) k (Suc q2))
  }

theorem mul_cancel_positive
      (a : Nat) (b : Nat) (x : Nat)
    : IsTrue (leq_nat (Suc Zero) x) → Equal Nat (mul a x) (mul b x) → Equal Nat a b =
  match x {
    Zero ↦ λhx. λh. absurd hx;
    Suc k ↦ λhx. λh. (proof cancel_r for mul) a b k h
  }

theorem add_cancel_right
      (a : Nat) (b : Nat)
    : (c : Nat) → Equal Nat (add a c) (add b c) → Equal Nat a b =
  λc.
    match c {
      Zero ↦ λh. h;
      Suc c2 ↦
        λh.
          add_cancel_right
            a
            b
            c2
            (cong Nat Nat (Suc (add a c2)) (Suc (add b c2)) (λn. sub n (Suc Zero)) h)
    }

theorem add_cancel_left
      (a : Nat) (b : Nat) (c : Nat) (h : Equal Nat (add a b) (add a c))
    : Equal Nat b c =
  add_cancel_right
    b
    c
    a
    (trans
      Nat
      (add b a)
      (add a b)
      (add c a)
      ((proof comm for add) b a)
      (trans Nat (add a b) (add a c) (add c a) h ((proof comm for add) a c)))

theorem remainder_unique
      (x : Nat) (a : Nat) (b : Nat) (m : Nat)
    : (n : Nat)
      → IsTrue (leq_nat (Suc a) x)
      → IsTrue (leq_nat (Suc b) x)
      → Equal Nat (add a (mul x m)) (add b (mul x n))
      → Equal Nat a b =
  match m {
    Zero ↦
      λn.
        match n {
          Zero ↦ λha. λhb. λh. h;
          Suc n2 ↦
            λha.
              λhb.
                λh.
                  absurd
                    (suc_leq_self
                      a
                      ((proof trans for leq_nat)
                        (Suc a)
                        x
                        a
                        ha
                        (J
                          (λy _. IsTrue (leq_nat x y))
                          ((proof trans for leq_nat)
                            x
                            (add (mul x n2) x)
                            (add b (add (mul x n2) x))
                            (leq_nat_add_right_bound (mul x n2) x)
                            (leq_nat_add_right_bound b (add (mul x n2) x)))
                          (sym Nat a (add b (add (mul x n2) x)) h))))
        };
    Suc m2 ↦
      λn.
        match n {
          Zero ↦
            λha.
              λhb.
                λh.
                  absurd
                    (suc_leq_self
                      b
                      ((proof trans for leq_nat)
                        (Suc b)
                        x
                        b
                        hb
                        (J
                          (λy _. IsTrue (leq_nat x y))
                          ((proof trans for leq_nat)
                            x
                            (add (mul x m2) x)
                            (add a (add (mul x m2) x))
                            (leq_nat_add_right_bound (mul x m2) x)
                            (leq_nat_add_right_bound a (add (mul x m2) x)))
                          h)));
          Suc n2 ↦
            λha.
              λhb.
                λh.
                  remainder_unique
                    x
                    a
                    b
                    m2
                    n2
                    ha
                    hb
                    (add_cancel_right
                      (add a (mul x m2))
                      (add b (mul x n2))
                      x
                      (trans
                        Nat
                        (add (add a (mul x m2)) x)
                        (add a (add (mul x m2) x))
                        (add (add b (mul x n2)) x)
                        (sym
                          Nat
                          (add a (add (mul x m2) x))
                          (add (add a (mul x m2)) x)
                          ((proof assoc for add) a (mul x m2) x))
                        (trans
                          Nat
                          (add a (add (mul x m2) x))
                          (add b (add (mul x n2) x))
                          (add (add b (mul x n2)) x)
                          h
                          ((proof assoc for add) b (mul x n2) x))))
        }
  }

theorem cons_positive
      (x : Nat)
      (g : Nat → Nat)
      (hx : IsTrue (leq_nat (Suc Zero) x))
      (hg : (v : Nat) → IsTrue (leq_nat (Suc Zero) (g v)))
    : (v : Nat) → IsTrue (leq_nat (Suc Zero) (cons_env x g v)) =
  λv.
    match v {
      Zero ↦ hx;
      Suc w ↦ hg w
    }

theorem shift_cons_eval
      (x : Nat) (g : Nat → Nat) (p : Poly)
    : Equal Nat (eval_poly (shift_env (cons_env x g)) p) (eval_poly g p) =
  cong (Nat → Nat) Nat (shift_env (cons_env x g)) g (λe. eval_poly e p) (λv. Refl)

theorem horner_positive
      (q : Poly)
    : (p0 : Poly)
      → (IsTrue
          (is_canonical p0)
          → Equal
          Bool
          (is_zero_poly p0)
          False
          → (env : Nat → Nat)
          → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
          → IsTrue
          (leq_nat (Suc Zero) (eval_poly env p0)))
      → (IsTrue
          (is_canonical q)
          → Equal
          Bool
          (is_zero_poly q)
          False
          → (env : Nat → Nat)
          → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
          → IsTrue
          (leq_nat (Suc Zero) (eval_poly env q)))
      → IsTrue (is_canonical (PHorner p0 q))
      → (env : Nat → Nat)
      → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
      → IsTrue (leq_nat (Suc Zero) (eval_poly env (PHorner p0 q))) =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          λp0.
            match p0 {
              PConst d ↦ λihp. λihq. λhc. absurd hc;
              PHorner a b ↦
                λihp.
                  λihq.
                    λhc.
                      λenv.
                        λhpos.
                          (proof trans for leq_nat)
                            (Suc Zero)
                            (eval_poly (shift_env env) (PHorner a b))
                            (add
                              (eval_poly (shift_env env) (PHorner a b))
                              (mul Zero (env Zero)))
                            (ihp
                              ((proof left for bool_and) (is_canonical (PHorner a b)) True hc)
                              Proved
                              (shift_env env)
                              (λv. hpos (Suc v)))
                            (leq_nat_add_left_bound
                              (eval_poly (shift_env env) (PHorner a b))
                              (mul Zero (env Zero)))
            };
        Suc c1 ↦
          λp0.
            λihp.
              λihq.
                λhc.
                  λenv.
                    λhpos.
                      (proof trans for leq_nat)
                        (Suc Zero)
                        (mul (Suc c1) (env Zero))
                        (add (eval_poly (shift_env env) p0) (mul (Suc c1) (env Zero)))
                        (mul_positive (Suc c1) (env Zero) Proved (hpos Zero))
                        (leq_nat_add_right_bound
                          (eval_poly (shift_env env) p0)
                          (mul (Suc c1) (env Zero)))
      };
    PHorner q1 q2 ↦
      λp0.
        λihp.
          λihq.
            λhc.
              λenv.
                λhpos.
                  (proof trans for leq_nat)
                    (Suc Zero)
                    (mul (eval_poly env (PHorner q1 q2)) (env Zero))
                    (add
                      (eval_poly (shift_env env) p0)
                      (mul (eval_poly env (PHorner q1 q2)) (env Zero)))
                    (mul_positive
                      (eval_poly env (PHorner q1 q2))
                      (env Zero)
                      (ihq
                        ((proof right for bool_and)
                          (is_canonical p0)
                          (is_canonical (PHorner q1 q2))
                          hc)
                        Proved
                        env
                        hpos)
                      (hpos Zero))
                    (leq_nat_add_right_bound
                      (eval_poly (shift_env env) p0)
                      (mul (eval_poly env (PHorner q1 q2)) (env Zero)))
  }

theorem canonical_positive
      (p : Poly)
    : IsTrue (is_canonical p)
      → Equal Bool (is_zero_poly p) False
      → (env : Nat → Nat)
      → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
      → IsTrue (leq_nat (Suc Zero) (eval_poly env p)) =
  match p {
    PConst c ↦
      match c {
        Zero ↦ λhc. λhz. absurd hz;
        Suc c1 ↦ λhc. λhz. λenv. λhpos. Proved
      };
    PHorner p0 q ↦
      λhc. λhz. horner_positive q p0 (canonical_positive p0) (canonical_positive q) hc
  }

theorem nonzero_horner_exceeds
      (a : Nat) (q : Nat) (b : Nat)
    : IsTrue (leq_nat (Suc Zero) q) → Equal Nat (add a (mul q (Suc b))) b → Bottom =
  λhq.
    λh.
      suc_leq_self
        b
        (J
          (λy _. IsTrue (leq_nat (Suc b) y))
          ((proof trans for leq_nat)
            (Suc b)
            (mul q (Suc b))
            (add a (mul q (Suc b)))
            (mul_ge_right q (Suc b) hq)
            (leq_nat_add_right_bound a (mul q (Suc b))))
          h)

theorem horner_const
      (q : Poly)
    : (p0 : Poly)
      → ((b : Nat)
          → IsTrue
          (is_canonical p0)
          → ((env : Nat → Nat)
            → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
            → Equal
            Nat
            (eval_poly env p0)
            b)
          → Equal
          Poly
          p0
          (PConst b))
      → (b : Nat)
      → IsTrue (is_canonical (PHorner p0 q))
      → ((env : Nat → Nat)
          → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
          → Equal
          Nat
          (eval_poly env (PHorner p0 q))
          b)
      → Equal Poly (PHorner p0 q) (PConst b) =
  match q {
    PConst c ↦
      match c {
        Zero ↦
          λp0.
            match p0 {
              PConst d ↦ λihc. λb. λhc. λh. absurd hc;
              PHorner a e ↦
                λihc.
                  λb.
                    λhc.
                      λh.
                        absurd
                          (ihc
                            b
                            ((proof left for bool_and) (is_canonical (PHorner a e)) True hc)
                            (λg.
                              λhg.
                                trans
                                  Nat
                                  (eval_poly g (PHorner a e))
                                  (eval_poly (shift_env (cons_env (Suc Zero) g)) (PHorner a e))
                                  b
                                  (sym
                                    Nat
                                    (eval_poly
                                      (shift_env (cons_env (Suc Zero) g))
                                      (PHorner a e))
                                    (eval_poly g (PHorner a e))
                                    (shift_cons_eval (Suc Zero) g (PHorner a e)))
                                  (h
                                    (cons_env (Suc Zero) g)
                                    (cons_positive (Suc Zero) g Proved hg))))
            };
        Suc c1 ↦
          λp0.
            λihc.
              λb.
                λhc.
                  λh.
                    absurd
                      (nonzero_horner_exceeds
                        (eval_poly (shift_env (cons_env (Suc b) one_env)) p0)
                        (Suc c1)
                        b
                        Proved
                        (h
                          (cons_env (Suc b) one_env)
                          (cons_positive (Suc b) one_env Proved (λv. Proved))))
      };
    PHorner q1 q2 ↦
      λp0.
        λihc.
          λb.
            λhc.
              λh.
                absurd
                  (nonzero_horner_exceeds
                    (eval_poly (shift_env (cons_env (Suc b) one_env)) p0)
                    (eval_poly (cons_env (Suc b) one_env) (PHorner q1 q2))
                    b
                    (canonical_positive
                      (PHorner q1 q2)
                      ((proof right for bool_and)
                        (is_canonical p0)
                        (is_canonical (PHorner q1 q2))
                        hc)
                      Proved
                      (cons_env (Suc b) one_env)
                      (cons_positive (Suc b) one_env Proved (λv. Proved)))
                    (h
                      (cons_env (Suc b) one_env)
                      (cons_positive (Suc b) one_env Proved (λv. Proved))))
  }

theorem const_complete
      (q : Poly)
    : (b : Nat)
      → IsTrue (is_canonical q)
      → ((env : Nat → Nat)
          → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
          → Equal
          Nat
          (eval_poly env q)
          b)
      → Equal Poly q (PConst b) =
  match q {
    PConst c ↦ λb. λhc. λh. cong Nat Poly c b PConst (h one_env (λv. Proved));
    PHorner p0 q0 ↦ horner_const q0 p0 (const_complete p0)
  }

theorem poly_complete
      (p : Poly)
    : (r : Poly)
      → IsTrue (is_canonical p)
      → IsTrue (is_canonical r)
      → ((env : Nat → Nat)
          → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
          → Equal
          Nat
          (eval_poly env p)
          (eval_poly env r))
      → Equal Poly p r =
  match p {
    PConst c ↦
      λr.
        λhp.
          λhr.
            λh.
              sym
                Poly
                r
                (PConst c)
                (const_complete r c hr (λenv. λhe. sym Nat c (eval_poly env r) (h env he)));
    PHorner p1 q1 ↦
      λr.
        match r {
          PConst d ↦ λhp. λhr. λh. const_complete (PHorner p1 q1) d hp h;
          PHorner p2 q2 ↦
            λhp.
              λhr.
                λh.
                  let
                    hc1 : IsTrue (bool_and (is_canonical p1) (is_canonical q1)) =
                      (proof right for bool_and)
                        (nondegenerate p1 q1)
                        (bool_and (is_canonical p1) (is_canonical q1))
                        hp;
                    hc2 : IsTrue (bool_and (is_canonical p2) (is_canonical q2)) =
                      (proof right for bool_and)
                        (nondegenerate p2 q2)
                        (bool_and (is_canonical p2) (is_canonical q2))
                        hr;
                    agree_head : (g : Nat → Nat)
                      → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (g v)))
                      → Equal Nat
                      (eval_poly g p1)
                      (eval_poly g p2) =
                      λg.
                        λhg.
                          let
                            separating_value : Nat =
                              Suc (add (eval_poly g p1) (eval_poly g p2));
                            separating_env : Nat → Nat = cons_env separating_value g
                          in
                            remainder_unique
                              separating_value
                              (eval_poly g p1)
                              (eval_poly g p2)
                              (eval_poly separating_env q1)
                              (eval_poly separating_env q2)
                              (leq_nat_add_left_bound (eval_poly g p1) (eval_poly g p2))
                              (leq_nat_add_right_bound (eval_poly g p1) (eval_poly g p2))
                              (trans
                                Nat
                                (add
                                  (eval_poly g p1)
                                  (mul separating_value (eval_poly separating_env q1)))
                                (add
                                  (eval_poly (shift_env separating_env) p1)
                                  (mul (eval_poly separating_env q1) separating_value))
                                (add
                                  (eval_poly g p2)
                                  (mul separating_value (eval_poly separating_env q2)))
                                (add_cong
                                  (eval_poly g p1)
                                  (eval_poly (shift_env separating_env) p1)
                                  (mul separating_value (eval_poly separating_env q1))
                                  (mul (eval_poly separating_env q1) separating_value)
                                  (sym
                                    Nat
                                    (eval_poly (shift_env separating_env) p1)
                                    (eval_poly g p1)
                                    (shift_cons_eval separating_value g p1))
                                  ((proof comm for mul)
                                    separating_value
                                    (eval_poly separating_env q1)))
                                (trans
                                  Nat
                                  (add
                                    (eval_poly (shift_env separating_env) p1)
                                    (mul (eval_poly separating_env q1) separating_value))
                                  (add
                                    (eval_poly (shift_env separating_env) p2)
                                    (mul (eval_poly separating_env q2) separating_value))
                                  (add
                                    (eval_poly g p2)
                                    (mul separating_value (eval_poly separating_env q2)))
                                  (h
                                    separating_env
                                    (cons_positive separating_value g Proved hg))
                                  (add_cong
                                    (eval_poly (shift_env separating_env) p2)
                                    (eval_poly g p2)
                                    (mul (eval_poly separating_env q2) separating_value)
                                    (mul separating_value (eval_poly separating_env q2))
                                    (shift_cons_eval separating_value g p2)
                                    ((proof comm for mul)
                                      (eval_poly separating_env q2)
                                      separating_value))));
                    agree_tail : (env : Nat → Nat)
                      → ((v : Nat) → IsTrue (leq_nat (Suc Zero) (env v)))
                      → Equal Nat
                      (eval_poly env q1)
                      (eval_poly env q2) =
                      λenv.
                        λhe.
                          mul_cancel_positive
                            (eval_poly env q1)
                            (eval_poly env q2)
                            (env Zero)
                            (he Zero)
                            (add_cancel_left
                              (eval_poly (shift_env env) p2)
                              (mul (eval_poly env q1) (env Zero))
                              (mul (eval_poly env q2) (env Zero))
                              (trans
                                Nat
                                (add
                                  (eval_poly (shift_env env) p2)
                                  (mul (eval_poly env q1) (env Zero)))
                                (add
                                  (eval_poly (shift_env env) p1)
                                  (mul (eval_poly env q1) (env Zero)))
                                (add
                                  (eval_poly (shift_env env) p2)
                                  (mul (eval_poly env q2) (env Zero)))
                                (cong
                                  Nat
                                  Nat
                                  (eval_poly (shift_env env) p2)
                                  (eval_poly (shift_env env) p1)
                                  (λt. add t (mul (eval_poly env q1) (env Zero)))
                                  (sym
                                    Nat
                                    (eval_poly (shift_env env) p1)
                                    (eval_poly (shift_env env) p2)
                                    (agree_head (shift_env env) (λv. he (Suc v)))))
                                (h env he)))
                  in
                    trans
                      Poly
                      (PHorner p1 q1)
                      (PHorner p2 q1)
                      (PHorner p2 q2)
                      (cong
                        Poly
                        Poly
                        p1
                        p2
                        (λt. PHorner t q1)
                        (poly_complete
                          p1
                          p2
                          ((proof left for bool_and) (is_canonical p1) (is_canonical q1) hc1)
                          ((proof left for bool_and) (is_canonical p2) (is_canonical q2) hc2)
                          agree_head))
                      (cong
                        Poly
                        Poly
                        q1
                        q2
                        (λt. PHorner p2 t)
                        (poly_complete
                          q1
                          q2
                          ((proof right for bool_and) (is_canonical p1) (is_canonical q1) hc1)
                          ((proof right for bool_and) (is_canonical p2) (is_canonical q2) hc2)
                          agree_tail))
        }
  }

theorem poly_same_sound (p : Poly) : (r : Poly) → IsTrue (poly_same p r) → Equal Poly p r =
  match p {
    PConst c ↦
      λr.
        match r {
          PConst d ↦
            λh.
              cong
                Nat
                Poly
                c
                d
                PConst
                ((proof antisym for leq_nat)
                  c
                  d
                  ((proof left for bool_and) (leq_nat c d) (leq_nat d c) h)
                  ((proof right for bool_and) (leq_nat c d) (leq_nat d c) h));
          PHorner r0 s ↦ λh. absurd h
        };
    PHorner p0 q ↦
      λr.
        match r {
          PConst d ↦ λh. absurd h;
          PHorner r0 s ↦
            λh.
              trans
                Poly
                (PHorner p0 q)
                (PHorner r0 q)
                (PHorner r0 s)
                (cong
                  Poly
                  Poly
                  p0
                  r0
                  (λt. PHorner t q)
                  (poly_same_sound
                    p0
                    r0
                    ((proof left for bool_and) (poly_same p0 r0) (poly_same q s) h)))
                (cong
                  Poly
                  Poly
                  q
                  s
                  (λt. PHorner r0 t)
                  (poly_same_sound
                    q
                    s
                    ((proof right for bool_and) (poly_same p0 r0) (poly_same q s) h)))
        }
  }

theorem poly_same_refl (p : Poly) : IsTrue (poly_same p p) =
  match p {
    PConst c ↦
      (proof intro for bool_and)
        (leq_nat c c)
        (leq_nat c c)
        ((proof refl for leq_nat) c)
        ((proof refl for leq_nat) c);
    PHorner p0 q ↦
      (proof intro for bool_and)
        (poly_same p0 p0)
        (poly_same q q)
        (poly_same_refl p0)
        (poly_same_refl q)
  }

fn var_deriv (b : Bool) : Expr =
  match b {
    True ↦ EConst (Suc Zero);
    False ↦ EConst Zero
  }

fn bump_at (b : Bool) (x : Nat) (t : Nat) : Nat =
  match b {
    True ↦ add x t;
    False ↦ x
  }

fn bump_env (v : Nat) (t : Nat) (env : Nat → Nat) (w : Nat) : Nat =
  bump_at (nat_same v w) (env w) t

fn taylor_rem (v : Nat) (t : Nat) (env : Nat → Nat) (e : Expr) : Nat =
  match e {
    EConst c ↦ Zero;
    EVar w ↦ Zero;
    EAdd a b ↦ add (taylor_rem v t env a) (taylor_rem v t env b);
    EMul a b ↦
      add
        (add
          (mul (taylor_rem v t env a) (eval env b))
          (mul (eval env a) (taylor_rem v t env b)))
        (mul
          (add (eval env (deriv v a)) (mul t (taylor_rem v t env a)))
          (add (eval env (deriv v b)) (mul t (taylor_rem v t env b))))
  }

theorem deriv_taylor
      (v : Nat) (t : Nat) (env : Nat → Nat) (e : Expr)
    : Equal Nat
        (eval (bump_env v t env) e)
        (add (eval env e) (mul t (add (eval env (deriv v e)) (mul t (taylor_rem v t env e))))) =
  match e {
    EConst c ↦ Refl;
    EVar w ↦ taylor_var (nat_same v w) env (env w) t;
    EAdd a b ↦
      trans
        Nat
        (add (eval (bump_env v t env) a) (eval (bump_env v t env) b))
        (add
          (add (eval env a) (mul t (add (eval env (deriv v a)) (mul t (taylor_rem v t env a)))))
          (add
            (eval env b)
            (mul t (add (eval env (deriv v b)) (mul t (taylor_rem v t env b))))))
        (add
          (add (eval env a) (eval env b))
          (mul
            t
            (add
              (add (eval env (deriv v a)) (eval env (deriv v b)))
              (mul t (add (taylor_rem v t env a) (taylor_rem v t env b))))))
        (add_cong
          (eval (bump_env v t env) a)
          (add (eval env a) (mul t (add (eval env (deriv v a)) (mul t (taylor_rem v t env a)))))
          (eval (bump_env v t env) b)
          (add (eval env b) (mul t (add (eval env (deriv v b)) (mul t (taylor_rem v t env b)))))
          (deriv_taylor v t env a)
          (deriv_taylor v t env b))
        (taylor_add_identity
          (eval env a)
          (eval env b)
          t
          (eval env (deriv v a))
          (eval env (deriv v b))
          (taylor_rem v t env a)
          (taylor_rem v t env b));
    EMul a b ↦
      trans
        Nat
        (mul (eval (bump_env v t env) a) (eval (bump_env v t env) b))
        (mul
          (add (eval env a) (mul t (add (eval env (deriv v a)) (mul t (taylor_rem v t env a)))))
          (add
            (eval env b)
            (mul t (add (eval env (deriv v b)) (mul t (taylor_rem v t env b))))))
        (add
          (mul (eval env a) (eval env b))
          (mul
            t
            (add
              (add
                (mul (eval env (deriv v a)) (eval env b))
                (mul (eval env a) (eval env (deriv v b))))
              (mul t (taylor_rem v t env (EMul a b))))))
        (mul_cong
          (eval (bump_env v t env) a)
          (add (eval env a) (mul t (add (eval env (deriv v a)) (mul t (taylor_rem v t env a)))))
          (eval (bump_env v t env) b)
          (add (eval env b) (mul t (add (eval env (deriv v b)) (mul t (taylor_rem v t env b)))))
          (deriv_taylor v t env a)
          (deriv_taylor v t env b))
        (taylor_mul_identity
          (eval env a)
          (eval env b)
          t
          (eval env (deriv v a))
          (eval env (deriv v b))
          (taylor_rem v t env a)
          (taylor_rem v t env b))
  }

theorem taylor_var
      (b : Bool) (env : Nat → Nat) (x : Nat) (t : Nat)
    : Equal Nat (bump_at b x t) (add x (mul t (add (eval env (var_deriv b)) (mul t Zero)))) =
  match b {
    True ↦
      poly_eq_sound
        (EAdd (EVar Zero) (EVar (Suc Zero)))
        (EAdd (EVar Zero) (EMul (EVar (Suc Zero)) (EConst (Suc Zero))))
        Proved
        (ring_env x t Zero Zero Zero Zero Zero);
    False ↦ Refl
  }

fn ring_env (a : Nat) (b : Nat) (c : Nat) (d : Nat) (e : Nat) (f : Nat) (g : Nat) : Nat → Nat =
  cons_env
    a
    (cons_env b (cons_env c (cons_env d (cons_env e (cons_env f (cons_env g one_env))))))

theorem taylor_add_identity
      (a : Nat) (b : Nat) (t : Nat) (da : Nat) (db : Nat) (ra : Nat) (rb : Nat)
    : Equal Nat
        (add (add a (mul t (add da (mul t ra)))) (add b (mul t (add db (mul t rb)))))
        (add (add a b) (mul t (add (add da db) (mul t (add ra rb))))) =
  poly_eq_sound
    (EAdd
      (EAdd (EVar Zero) (EMul ring_t (EAdd ring_da (EMul ring_t ring_ra))))
      (EAdd (EVar (Suc Zero)) (EMul ring_t (EAdd ring_db (EMul ring_t ring_rb)))))
    (EAdd
      (EAdd (EVar Zero) (EVar (Suc Zero)))
      (EMul ring_t (EAdd (EAdd ring_da ring_db) (EMul ring_t (EAdd ring_ra ring_rb)))))
    Proved
    (ring_env a b t da db ra rb)

theorem taylor_mul_identity
      (a : Nat) (b : Nat) (t : Nat) (da : Nat) (db : Nat) (ra : Nat) (rb : Nat)
    : Equal Nat
        (mul (add a (mul t (add da (mul t ra)))) (add b (mul t (add db (mul t rb)))))
        (add
          (mul a b)
          (mul
            t
            (add
              (add (mul da b) (mul a db))
              (mul
                t
                (add
                  (add (mul ra b) (mul a rb))
                  (mul (add da (mul t ra)) (add db (mul t rb)))))))) =
  poly_eq_sound
    (EMul
      (EAdd (EVar Zero) (EMul ring_t (EAdd ring_da (EMul ring_t ring_ra))))
      (EAdd (EVar (Suc Zero)) (EMul ring_t (EAdd ring_db (EMul ring_t ring_rb)))))
    (EAdd
      (EMul (EVar Zero) (EVar (Suc Zero)))
      (EMul
        ring_t
        (EAdd
          (EAdd (EMul ring_da (EVar (Suc Zero))) (EMul (EVar Zero) ring_db))
          (EMul
            ring_t
            (EAdd
              (EAdd (EMul ring_ra (EVar (Suc Zero))) (EMul (EVar Zero) ring_rb))
              (EMul
                (EAdd ring_da (EMul ring_t ring_ra))
                (EAdd ring_db (EMul ring_t ring_rb))))))))
    Proved
    (ring_env a b t da db ra rb)

const ring_t : Expr = EVar (Suc (Suc Zero))

const ring_da : Expr = EVar (Suc (Suc (Suc Zero)))

const ring_db : Expr = EVar (Suc (Suc (Suc (Suc Zero))))

const ring_ra : Expr = EVar (Suc (Suc (Suc (Suc (Suc Zero)))))

const ring_rb : Expr = EVar (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))

theorem deriv_extract
      (a1 : Nat)
      (a2 : Nat)
      (t : Nat)
      (d1 : Nat)
      (d2 : Nat)
      (r1 : Nat)
      (r2 : Nat)
      (ht : IsTrue (leq_nat (Suc Zero) t))
      (h1 : IsTrue (leq_nat (Suc d1) t))
      (h2 : IsTrue (leq_nat (Suc d2) t))
      (h0 : Equal Nat a1 a2)
      (h : Equal Nat (add a1 (mul t (add d1 (mul t r1)))) (add a2 (mul t (add d2 (mul t r2)))))
    : Equal Nat d1 d2 =
  remainder_unique
    t
    d1
    d2
    r1
    r2
    h1
    h2
    (mul_cancel_positive
      (add d1 (mul t r1))
      (add d2 (mul t r2))
      t
      ht
      (trans
        Nat
        (mul (add d1 (mul t r1)) t)
        (mul t (add d1 (mul t r1)))
        (mul (add d2 (mul t r2)) t)
        ((proof comm for mul) (add d1 (mul t r1)) t)
        (trans
          Nat
          (mul t (add d1 (mul t r1)))
          (mul t (add d2 (mul t r2)))
          (mul (add d2 (mul t r2)) t)
          (add_cancel_left
            a1
            (mul t (add d1 (mul t r1)))
            (mul t (add d2 (mul t r2)))
            (trans
              Nat
              (add a1 (mul t (add d1 (mul t r1))))
              (add a2 (mul t (add d2 (mul t r2))))
              (add a1 (mul t (add d2 (mul t r2))))
              h
              (cong Nat Nat a2 a1 (λu. add u (mul t (add d2 (mul t r2)))) (sym Nat a1 a2 h0))))
          ((proof comm for mul) t (add d2 (mul t r2))))))
```

## 3. Using it

Construct an expression from the exported constructors and use a total
natural-valued environment. `normalize` distributes first and then reifies;
its shape retains summand order and the terminal zero expression. The
following client of the package's functions states a checked evaluation law.

```ken example
fn example_env (v : Nat) : Nat = Suc v

const example_expr : Expr = EAdd (EVar Zero) (EConst (Suc Zero))

theorem example_normalize_sound
    : Equal Nat (eval example_env (normalize example_expr)) (eval example_env example_expr) =
  normalize_sound example_env example_expr
```

Use `canonicalize` when structural agreement of the distributed list is not
enough. The following two expressions differ syntactically but represent the
same polynomial; their canonical forms agree and `poly_eq` answers `True`.

```ken example
const double_variable_by_addition : Expr = EAdd (EVar Zero) (EVar Zero)

const double_variable_by_product : Expr = EMul (EConst (Suc (Suc Zero))) (EVar Zero)

theorem example_canonical_equality
    : Equal Poly
        (canonicalize double_variable_by_addition)
        (canonicalize double_variable_by_product) =
  Refl

theorem example_decision
    : IsTrue (poly_eq double_variable_by_addition double_variable_by_product) =
  Proved
```

## 4. Laws & proofs

`monomials_reify` round-trips every ordered monomial list structurally. Its
private monomial-expression proof uses the predecessor coefficient spelling
that leaves a unit variable product unchanged. `normalize_idempotent` lifts
this round trip under `reify`, so repeated normalization has the same exact
`Expr` structure.

`monomials_sound` proceeds by structural induction on expressions. The
addition case uses a checked append-evaluation bridge. The multiplication
case distributes each left monomial across every right monomial and uses
separate checked proofs for coefficient multiplication, variable-list
concatenation, scaled batches and their sum. `reify_sound` proves its own
list induction using the monomial-expression evaluation lemma.
`normalize_sound` composes the two evaluation laws. These are proofs for
arbitrary expressions, lists and environments; the closed example only
illustrates one use site.

`canonicalize_sound` relates the Horner evaluator to the expression evaluator.
`canonicalize_complete` proves the converse at the form level: agreement under
all total environments identifies the same canonical `Poly`. The private
canonicity predicate and positivity lemmas prove that a nonzero canonical
polynomial remains positive under positive environments. The completeness
proof separates Horner heads with a sufficiently large value for variable
zero, cancels the positive multiplier to identify tails, and recurses over
both children. `poly_eq_sound` and `poly_eq_complete` link structural comparison
to semantic equality; `poly_eq_false_refutes` rules out a false answer when a
proof of agreement under every environment is supplied. It does not construct
an environment witnessing inequality.

`eval_substitute` follows the expression constructors: constant and variable
cases reduce directly, while the sum and product cases lift both induction
hypotheses with private addition and multiplication congruence. In
`poly_eq_substitute`, `poly_eq_sound` supplies equality under the composed
environment; `eval_substitute` transports each side to the substituted
expression, and `poly_eq_complete` yields the `True` decision. No proof of a
reverse implication is used or claimed. `substitute_compose` inducts over the
original expression and lifts the induction hypotheses through `EAdd` and
`EMul` with private congruence proofs. `substitute_var` uses the same structural
steps with `EVar` as the identity map; neither theorem equates functions.

`deriv_respects_eval` proves a Taylor expansion by structural induction on an
expression, with a private natural-valued remainder. The arithmetic in its
addition and multiplication branches follows from `poly_eq_sound` applied to
polynomial identities. Equal evaluations at the original and bumped
environments cancel the base value and positive bump factor. Taking the bump
larger than both derivative values lets `remainder_unique` identify those
values. `poly_eq_deriv` applies this semantic result to `poly_eq_sound`, then
uses `poly_eq_complete` for the decision. Neither law equates derivative
expression trees or uses `canonicalize_complete` as a derivative theorem.

## 5. Design notes

A variable index is a `Nat`, and the evaluator's environment is a total
`Nat → Nat` function. Multiplying `Mono a vs` by `Mono b ws` concatenates
`vs` before `ws` and computes the predecessor coefficient as
`add b (mul (Suc b) a)`. This form makes the exact list round trip reducible
without assuming a new arithmetic rule. Distribution retains repetitions
and order; equal evaluation does not imply equal monomial lists. In
particular, `x₀+x₀` and `2·x₀` can evaluate equally but remain structurally
distinct. The §1 `normalize : Expr → Expr` remains this ordered, noncanonical
operation; §2's separate `canonicalize : Expr → Poly` merges like terms.
Substitution preserves the original expression constructors and inserts each
variable's image once;
evaluating the original expression under `compose_env` gives the same result
as evaluating its substitution under the given environment. To apply two
substitutions, `compose_subst s t` visits each `t` image with `s`; it does not
change the single-pass behavior of either substitution. Differentiation of a
product keeps both summands in left-then-right order; derivatives of different
expressions with equal evaluations need not have identical expression trees.

A `PHorner p q` reads `p` with variable indices shifted upward and adds `q`
multiplied by variable zero. Collapsing `PHorner (PConst c) (PConst Zero)`
removes a redundant constant node. The zero second child must remain when the
first child is nonconstant, because shifting a variable still changes its
index. Public constructors may build noncanonical values; `canonicalize`
produces canonical ones. Comparison uses both natural-number order directions
for coefficient equality, not a new trusted equality primitive.

## 6. References

- [Polynomial ring](https://en.wikipedia.org/wiki/Polynomial_ring) —
  Wikipedia; background on polynomial terms and coefficient domains. The
  natural-number carrier here is not a field and this representation does
  not collect terms.
- [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method) —
  Wikipedia; orientation on nested polynomial representations. This module's
  recursive form indexes multiple variables and is not an evaluation-cost
  guarantee.

## 7. Trust & derivation

The [contract](../../../../spec/50-stdlib/66-symbolic-algebra.md) §§1–5 and
its [distributed seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md),
[canonical-equality seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-canonical-equality.md),
[substitution seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution.md),
[composition seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution-composition.md),
and [derivative seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-derivative.md)
specify the public surface and closed observations. The public API comprises
`Expr`, `Monomial`, `Poly`, their exported constructors, thirteen operations,
and sixteen checked theorems; Horner, substitution and differentiation expose
no additional helper beyond those named operations and laws. Definition
locates the implementations and law statements; Using it gives client idioms;
Laws & proofs traces the proof families; Design notes separates their
representations and limits.

The source derives evaluation from the checked natural arithmetic, logic
transport, Boolean conjunction, natural order and list-append definitions.
Private functions and lemmas implement distribution, Horner collection,
canonicity, positivity, substitution, differentiation and expression
congruence, and proof rearrangements.
The root-loaded acceptance target compares exact trusted-`GlobalId` sets
before and after loading the
package on its providers. The extension adds no Axiom, postulate, primitive,
or foreign operation; inherited trust from the prelude and dependencies
remains distinct from local trust.
