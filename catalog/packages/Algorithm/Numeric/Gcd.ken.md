# Verified Euclidean greatest common divisor

This package presents Euclid's algorithm over unary natural numbers.  Its
headline operation is `divides_gcd`: every common divisor of the two inputs
also divides their computed gcd.  An explicit structural fuel argument makes
the subtraction-based decrease visible to Ken's termination checker.

## Definition

The package reuses canonical addition and multiplication from `Nat` arithmetic
and reaches the canonical `leq_nat` identity through the `Nat` order facade.
Saturating subtraction is supplied by that facade.  The three data carriers
come first in loader dependency order; `divides_gcd` is then the first function
in the uninterrupted checked function/proof run, followed by its increasingly
fundamental implementation support.

```ken
import Data.Numeric.Nat.Arithmetic (add, mul)

import Data.Numeric.Nat.Order (leq_nat, sub)

data Divides (d : Nat) (n : Nat) : Type where {
  MkDivides : (quotient : Nat) → Equal Nat n (mul d quotient) → Divides d n
}

data BoolView (value : Bool) : Type where {
  BoolIsTrue : Equal Bool value True → BoolView value;
  BoolIsFalse : Equal Bool value False → BoolView value
}

data GcdSpec (a : Nat) (b : Nat) (g : Nat) : Type where {
  MkGcdSpec :
    Divides g a
    → Divides g b
    → ((d : Nat) → Divides d a → Divides d b → Divides d g)
    → GcdSpec a b g
}

fn divides_gcd
      (d : Nat) (a : Nat) (b : Nat) (da : Divides d a) (db : Divides d b)
    : Divides d (gcd a b) =
  gcd_spec_greatest a b (gcd a b) (gcd_spec a b) d da db

fn gcd_divides_left (a : Nat) (b : Nat) : Divides (gcd a b) a =
  gcd_spec_left a b (gcd a b) (gcd_spec a b)

fn gcd_divides_right (a : Nat) (b : Nat) : Divides (gcd a b) b =
  gcd_spec_right a b (gcd a b) (gcd_spec a b)

fn gcd_spec (a : Nat) (b : Nat) : GcdSpec a b (gcd a b) =
  gcd_fuel_spec (Suc (add a b)) a b (leq_weaken_right (add a b) (add a b) (leq_refl (add a b)))

fn gcd_fuel_spec
      (fuel : Nat)
    : (a : Nat)
      → (b : Nat)
      → Equal Bool (leq_nat (add a b) fuel) True
      → GcdSpec a b (gcd_fuel fuel a b) =
  match fuel {
    Zero ↦
      λa.
        λb.
          match b {
            Zero ↦
              match a {
                Zero ↦
                  λbound.
                    MkGcdSpec
                      Zero
                      Zero
                      Zero
                      (divides_zero Zero)
                      (divides_zero Zero)
                      (λd. λda. λdb. da);
                Suc a2 ↦ λbound. absurd bound
              };
            Suc b2 ↦ λbound. absurd bound
          };
    Suc fuel2 ↦
      λa.
        match a {
          Zero ↦
            λb. λbound. MkGcdSpec Zero b b (divides_zero b) (divides_self b) (λd. λda. λdb. db);
          Suc a2 ↦
            λb.
              match b {
                Zero ↦
                  λbound.
                    MkGcdSpec
                      (Suc a2)
                      Zero
                      (Suc a2)
                      (divides_self (Suc a2))
                      (divides_zero (Suc a2))
                      (λd. λda. λdb. da);
                Suc b2 ↦
                  λbound.
                    match bool_view (leq_nat (Suc a2) (Suc b2)) {
                      BoolIsTrue order ↦
                        let
                          g = gcd_fuel fuel2 (Suc a2) (sub (Suc b2) (Suc a2));
                          recurse =
                            gcd_fuel_spec
                              fuel2
                              (Suc a2)
                              (sub (Suc b2) (Suc a2))
                              (fuel_bound_sub_right a2 b2 fuel2 order bound);
                          ga = gcd_spec_left (Suc a2) (sub (Suc b2) (Suc a2)) g recurse;
                          gr = gcd_spec_right (Suc a2) (sub (Suc b2) (Suc a2)) g recurse;
                          greatest =
                            gcd_spec_greatest (Suc a2) (sub (Suc b2) (Suc a2)) g recurse;
                          branch_spec =
                            MkGcdSpec
                              (Suc a2)
                              (Suc b2)
                              g
                              ga
                              (subst_divides
                                g
                                (add (sub (Suc b2) (Suc a2)) (Suc a2))
                                (Suc b2)
                                (add_sub_cancel_leq (Suc a2) (Suc b2) order)
                                (divides_add g (sub (Suc b2) (Suc a2)) (Suc a2) gr ga))
                              (λd.
                                λda. λdb. greatest d da (divides_sub d (Suc b2) (Suc a2) db da))
                        in
                          J
                            (λdecision _.
                              GcdSpec
                                (Suc a2)
                                (Suc b2)
                                (match decision {
                                  True ↦ gcd_fuel fuel2 (Suc a2) (sub (Suc b2) (Suc a2));
                                  False ↦ gcd_fuel fuel2 (sub (Suc a2) (Suc b2)) (Suc b2)
                                }))
                            branch_spec
                            (sym Bool (leq_nat (Suc a2) (Suc b2)) True order);
                      BoolIsFalse order ↦
                        let
                          g = gcd_fuel fuel2 (sub (Suc a2) (Suc b2)) (Suc b2);
                          recurse =
                            gcd_fuel_spec
                              fuel2
                              (sub (Suc a2) (Suc b2))
                              (Suc b2)
                              (fuel_bound_sub_left a2 b2 fuel2 order bound);
                          gr = gcd_spec_left (sub (Suc a2) (Suc b2)) (Suc b2) g recurse;
                          gb = gcd_spec_right (sub (Suc a2) (Suc b2)) (Suc b2) g recurse;
                          greatest =
                            gcd_spec_greatest (sub (Suc a2) (Suc b2)) (Suc b2) g recurse;
                          branch_spec =
                            MkGcdSpec
                              (Suc a2)
                              (Suc b2)
                              g
                              (subst_divides
                                g
                                (add (sub (Suc a2) (Suc b2)) (Suc b2))
                                (Suc a2)
                                (add_sub_cancel_leq
                                  (Suc b2)
                                  (Suc a2)
                                  (leq_not_flip (Suc a2) (Suc b2) order))
                                (divides_add g (sub (Suc a2) (Suc b2)) (Suc b2) gr gb))
                              gb
                              (λd.
                                λda. λdb. greatest d (divides_sub d (Suc a2) (Suc b2) da db) db)
                        in
                          J
                            (λdecision _.
                              GcdSpec
                                (Suc a2)
                                (Suc b2)
                                (match decision {
                                  True ↦ gcd_fuel fuel2 (Suc a2) (sub (Suc b2) (Suc a2));
                                  False ↦ gcd_fuel fuel2 (sub (Suc a2) (Suc b2)) (Suc b2)
                                }))
                            branch_spec
                            (sym Bool (leq_nat (Suc a2) (Suc b2)) False order)
                    }
              }
        }
  }

fn gcd_spec_greatest
      (a : Nat) (b : Nat) (g : Nat) (spec : GcdSpec a b g)
    : (d : Nat) → Divides d a → Divides d b → Divides d g =
  match spec {
    MkGcdSpec left right greatest ↦ greatest
  }

fn gcd_spec_right (a : Nat) (b : Nat) (g : Nat) (spec : GcdSpec a b g) : Divides g b =
  match spec {
    MkGcdSpec left right greatest ↦ right
  }

fn gcd_spec_left (a : Nat) (b : Nat) (g : Nat) (spec : GcdSpec a b g) : Divides g a =
  match spec {
    MkGcdSpec left right greatest ↦ left
  }

fn gcd (a : Nat) (b : Nat) : Nat = gcd_fuel (Suc (add a b)) a b

fn gcd_fuel (fuel : Nat) (a : Nat) (b : Nat) : Nat =
  match fuel {
    Zero ↦ Zero;
    Suc fuel2 ↦
      match a {
        Zero ↦ b;
        Suc a2 ↦
          match b {
            Zero ↦ a;
            Suc b2 ↦
              match leq_nat a b {
                True ↦ gcd_fuel fuel2 a (sub b a);
                False ↦ gcd_fuel fuel2 (sub a b) b
              }
          }
      }
  }

theorem fuel_bound_sub_right
      (a : Nat)
      (b : Nat)
      (fuel : Nat)
      (order : Equal Bool (leq_nat (Suc a) (Suc b)) True)
      (bound : Equal Bool (leq_nat (add (Suc a) (Suc b)) (Suc fuel)) True)
    : Equal Bool (leq_nat (add (Suc a) (sub (Suc b) (Suc a))) fuel) True =
  trans
    Bool
    (leq_nat (add (Suc a) (sub (Suc b) (Suc a))) fuel)
    (leq_nat (Suc b) fuel)
    True
    (cong
      Nat
      Bool
      (add (Suc a) (sub (Suc b) (Suc a)))
      (Suc b)
      (λz. leq_nat z fuel)
      (trans
        Nat
        (add (Suc a) (sub (Suc b) (Suc a)))
        (add (sub (Suc b) (Suc a)) (Suc a))
        (Suc b)
        (add_comm (Suc a) (sub (Suc b) (Suc a)))
        (add_sub_cancel_leq (Suc a) (Suc b) order)))
    (leq_right_of_positive_sum a (Suc b) fuel bound)

theorem fuel_bound_sub_left
      (a : Nat)
      (b : Nat)
      (fuel : Nat)
      (order : Equal Bool (leq_nat (Suc a) (Suc b)) False)
      (bound : Equal Bool (leq_nat (add (Suc a) (Suc b)) (Suc fuel)) True)
    : Equal Bool (leq_nat (add (sub (Suc a) (Suc b)) (Suc b)) fuel) True =
  trans
    Bool
    (leq_nat (add (sub (Suc a) (Suc b)) (Suc b)) fuel)
    (leq_nat (Suc a) fuel)
    True
    (cong
      Nat
      Bool
      (add (sub (Suc a) (Suc b)) (Suc b))
      (Suc a)
      (λz. leq_nat z fuel)
      (add_sub_cancel_leq (Suc b) (Suc a) (leq_not_flip (Suc a) (Suc b) order)))
    (leq_left_of_sum (Suc a) b fuel bound)

theorem leq_right_of_positive_sum
      (a : Nat)
      (b : Nat)
      (bound : Nat)
      (h : Equal Bool (leq_nat (add (Suc a) b) (Suc bound)) True)
    : Equal Bool (leq_nat b bound) True =
  leq_left_of_sum
    b
    a
    bound
    (trans
      Bool
      (leq_nat (add b a) bound)
      (leq_nat (add (Suc a) b) (Suc bound))
      True
      (sym
        Bool
        (leq_nat (add (Suc a) b) (Suc bound))
        (leq_nat (add b a) bound)
        (cong
          Nat
          Bool
          (add (Suc a) b)
          (add b (Suc a))
          (λz. leq_nat z (Suc bound))
          (add_comm (Suc a) b)))
      h)

theorem leq_left_of_sum
      (a : Nat)
    : (b : Nat)
      → (bound : Nat)
      → Equal Bool (leq_nat (add a b) bound) True
      → Equal Bool (leq_nat a bound) True =
  λb.
    match b {
      Zero ↦ λbound. λh. h;
      Suc b2 ↦
        λbound.
          match bound {
            Zero ↦ λh. absurd h;
            Suc bound2 ↦ λh. leq_weaken_right a bound2 (leq_left_of_sum a b2 bound2 h)
          }
    }

theorem leq_weaken_right
      (a : Nat)
    : (b : Nat) → Equal Bool (leq_nat a b) True → Equal Bool (leq_nat a (Suc b)) True =
  match a {
    Zero ↦ λb. λh. Proved;
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦ λh. leq_weaken_right a2 b2 h
        }
  }

theorem leq_refl (a : Nat) : Equal Bool (leq_nat a a) True =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦ leq_refl a2
  }

theorem add_sub_cancel_leq
      (a : Nat)
    : (b : Nat) → Equal Bool (leq_nat a b) True → Equal Nat (add (sub b a) a) b =
  match a {
    Zero ↦ λb. λh. Refl;
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦ λh. cong Nat Nat (add (sub b2 a2) a2) b2 Suc (add_sub_cancel_leq a2 b2 h)
        }
  }

theorem leq_not_flip
      (a : Nat)
    : (b : Nat) → Equal Bool (leq_nat a b) False → Equal Bool (leq_nat b a) True =
  match a {
    Zero ↦ λb. λh. absurd (sym Bool True False h);
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. Proved;
          Suc b2 ↦ λh. leq_not_flip a2 b2 h
        }
  }

fn divides_sub
      (d : Nat) (x : Nat) (y : Nat) (dx : Divides d x) (dy : Divides d y)
    : Divides d (sub x y) =
  match dx {
    MkDivides qx ex ↦
      match dy {
        MkDivides qy ey ↦
          MkDivides
            d
            (sub x y)
            (sub qx qy)
            (trans
              Nat
              (sub x y)
              (sub (mul d qx) y)
              (mul d (sub qx qy))
              (cong Nat Nat x (mul d qx) (λz. sub z y) ex)
              (trans
                Nat
                (sub (mul d qx) y)
                (sub (mul d qx) (mul d qy))
                (mul d (sub qx qy))
                (cong Nat Nat y (mul d qy) (λz. sub (mul d qx) z) ey)
                (mul_sub d qx qy)))
      }
  }

fn divides_add
      (d : Nat) (x : Nat) (y : Nat) (dx : Divides d x) (dy : Divides d y)
    : Divides d (add x y) =
  match dx {
    MkDivides qx ex ↦
      match dy {
        MkDivides qy ey ↦
          MkDivides
            d
            (add x y)
            (add qx qy)
            (trans
              Nat
              (add x y)
              (add (mul d qx) y)
              (mul d (add qx qy))
              (cong Nat Nat x (mul d qx) (λz. add z y) ex)
              (trans
                Nat
                (add (mul d qx) y)
                (add (mul d qx) (mul d qy))
                (mul d (add qx qy))
                (cong Nat Nat y (mul d qy) (λz. add (mul d qx) z) ey)
                (sym Nat (mul d (add qx qy)) (add (mul d qx) (mul d qy)) (mul_add d qx qy))))
      }
  }

fn divides_self (d : Nat) : Divides d d =
  match d {
    Zero ↦ MkDivides Zero Zero Zero Proved;
    Suc d2 ↦
      MkDivides
        (Suc d2)
        (Suc d2)
        (Suc Zero)
        (cong Nat Nat d2 (add Zero d2) Suc (sym Nat (add Zero d2) d2 (add_zero_left d2)))
  }

fn divides_zero (d : Nat) : Divides d Zero = MkDivides d Zero Zero Proved

fn subst_divides
      (d : Nat) (x : Nat) (y : Nat) (p : Equal Nat x y) (dx : Divides d x)
    : Divides d y =
  subst Nat x y (λn. Divides d n) p dx

theorem mul_sub
      (d : Nat) (x : Nat) (y : Nat)
    : Equal Nat (sub (mul d x) (mul d y)) (mul d (sub x y)) =
  match y {
    Zero ↦ Refl;
    Suc y2 ↦
      match x {
        Zero ↦ sub_zero_left (mul d (Suc y2));
        Suc x2 ↦
          trans
            Nat
            (sub (add (mul d x2) d) (add (mul d y2) d))
            (sub (mul d x2) (mul d y2))
            (mul d (sub x2 y2))
            (sub_add_same (mul d x2) (mul d y2) d)
            (mul_sub d x2 y2)
      }
  }

theorem sub_add_same
      (x : Nat) (y : Nat) (z : Nat)
    : Equal Nat (sub (add x z) (add y z)) (sub x y) =
  match z {
    Zero ↦ Refl;
    Suc z2 ↦ sub_add_same x y z2
  }

theorem sub_zero_left (n : Nat) : Equal Nat (sub Zero n) Zero =
  match n {
    Zero ↦ Proved;
    Suc n2 ↦ Proved
  }

theorem mul_add
      (d : Nat) (x : Nat) (y : Nat)
    : Equal Nat (mul d (add x y)) (add (mul d x) (mul d y)) =
  match y {
    Zero ↦ Refl;
    Suc y2 ↦
      trans
        Nat
        (add (mul d (add x y2)) d)
        (add (add (mul d x) (mul d y2)) d)
        (add (mul d x) (add (mul d y2) d))
        (cong
          Nat
          Nat
          (mul d (add x y2))
          (add (mul d x) (mul d y2))
          (λz. add z d)
          (mul_add d x y2))
        (sym
          Nat
          (add (mul d x) (add (mul d y2) d))
          (add (add (mul d x) (mul d y2)) d)
          (add_assoc (mul d x) (mul d y2) d))
  }

theorem add_comm (a : Nat) (b : Nat) : Equal Nat (add a b) (add b a) =
  match b {
    Zero ↦ sym Nat (add Zero a) a (add_zero_left a);
    Suc b2 ↦
      trans
        Nat
        (add a (Suc b2))
        (Suc (add b2 a))
        (add (Suc b2) a)
        (cong Nat Nat (add a b2) (add b2 a) Suc (add_comm a b2))
        (sym Nat (add (Suc b2) a) (Suc (add b2 a)) (add_suc_left b2 a))
  }

theorem add_assoc
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (add a (add b c)) (add (add a b) c) =
  match c {
    Zero ↦ Refl;
    Suc c2 ↦ cong Nat Nat (add a (add b c2)) (add (add a b) c2) Suc (add_assoc a b c2)
  }

theorem add_suc_left (a : Nat) (b : Nat) : Equal Nat (add (Suc a) b) (Suc (add a b)) =
  match b {
    Zero ↦ Refl;
    Suc b2 ↦ cong Nat Nat (add (Suc a) b2) (Suc (add a b2)) Suc (add_suc_left a b2)
  }

theorem add_zero_left (a : Nat) : Equal Nat (add Zero a) a =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦ cong Nat Nat (add Zero a2) a2 Suc (add_zero_left a2)
  }

fn bool_view (value : Bool) : BoolView value =
  match value {
    True ↦ BoolIsTrue True Proved;
    False ↦ BoolIsFalse False Proved
  }

theorem trans
      (ty : Type) (x : ty) (y : ty) (z : ty) (p : Equal ty x y) (q : Equal ty y z)
    : Equal ty x z =
  J (λz2 _. Equal ty x z2) p q

theorem sym (ty : Type) (x : ty) (y : ty) (p : Equal ty x y) : Equal ty y x =
  J (λy2 _. Equal ty y2 x) Refl p

theorem cong
      (ty : Type) (ty2 : Type) (x : ty) (y : ty) (f : ty → ty2) (p : Equal ty x y)
    : Equal ty2 (f x) (f y) =
  J (λy2 _. Equal ty2 (f x) (f y2)) Refl p

fn subst
      (ty : Type) (x : ty) (y : ty) (fam : ty → Type) (p : Equal ty x y) (px : fam x)
    : fam y =
  J (λy2 _. fam y2) px p
```

## Laws and proofs

`gcd_fuel_spec` packages all three obligations in one invariant.  Its
specializations `gcd_divides_left` and `gcd_divides_right` produce the two
quotient witnesses required by the divisibility contract.  Given quotient
witnesses for any common divisor, the headline `divides_gcd` constructs the
witness that the divisor also divides the computed gcd.  These declarations
are `fn` rather than `theorem` because `Divides` deliberately retains its
quotient as proof-relevant data in `Type`.

## Trust and derivation

`gcd_fuel` is structural on its explicit fuel.  The public fuel is one more
than the sum of the inputs; every positive subtraction step strictly lowers
that sum.  `Divides` is witness-bearing data, and the required laws are checked
proof terms.  `add` and `mul` retain their Arithmetic identities, `leq_nat`
retains its LawfulClasses identity through the Order facade, and `sub` retains
its Order identity.  This package introduces no axiom, postulate, primitive,
foreign declaration, or local replacement for those providers.
