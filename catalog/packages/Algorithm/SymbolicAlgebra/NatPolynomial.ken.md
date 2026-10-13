# Natural-coefficient polynomials

Ordered monomials expose the structure produced by distributing a polynomial
expression over natural numbers. The five laws are kernel-checked generic proofs.

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
monomial. The representation does not sort variables or collect like terms.

## 2. Definition

`Expr` builds constants, variables, sums and products. `Monomial` pairs a
positive coefficient with an ordered list of variable indices. The public
operations interpret either carrier, distribute products into monomial lists,
reify those lists and normalize by reification. The carriers and their five
constructors are exported alongside the public functions and laws; internal
products and proof bridges remain private. Public laws lead the definition,
so a reader can find each contract before its supporting proofs.

```ken
import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l, mul_add_distrib_r)

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

## 5. Design notes

A variable index is a `Nat`, and the evaluator's environment is a total
`Nat → Nat` function. Multiplying `Mono a vs` by `Mono b ws` concatenates
`vs` before `ws` and computes the predecessor coefficient as
`add b (mul (Suc b) a)`. This form makes the exact list round trip reducible
without assuming a new arithmetic rule. Distribution retains repetitions
and order; equal evaluation does not imply equal monomial lists. In
particular, `x₀+x₀` and `2·x₀` can evaluate equally but remain structurally
distinct. No canonicalization or semantic-equality decision is promised.

## 6. References

- [Polynomial ring](https://en.wikipedia.org/wiki/Polynomial_ring) —
  Wikipedia; background on polynomial terms and coefficient domains. The
  natural-number carrier here is not a field and this representation does
  not collect terms.

## 7. Trust & derivation

The [contract](../../../../spec/50-stdlib/66-symbolic-algebra.md) §1 and
[seed](../../../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md)
specify the public surface and closed observations. Public names are `Expr`,
`Monomial`, their five constructors, six functions and five theorems. The
source uses checked `Core.Logic.Transport`, `Data.Numeric.Nat.Arithmetic`
and `Data.Collections.Derived` definitions; its sixteen private functions
and lemmas handle products, lists and proof rearrangements. The exact
`trusted_base()` comparison belongs to the root-loaded acceptance target.
No new Axiom, postulate, primitive or `Float` is introduced; inherited
trust from the prelude and dependencies remains distinct from local trust.
