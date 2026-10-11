# `Nat` arithmetic — canonical operations and free algebraic laws

Natural-number addition and multiplication compute by structural recursion.
Their familiar identity, commutativity, associativity, and distributivity laws
are introduced in prose before the dependency-first checked definitions and
proof terms that establish them.

## Operations and laws

The two computational names are introduced first because they occur in every
law's type. The checked declarations remain dependency-first: each attached
proof follows the operation whose public theory it belongs to and uses that
operation's `S::name` namespace for recursive and cross-law references.

```ken
import Core.Logic.Transport (cong, sym, trans)

pub fn add (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ a;
    Suc b2 ↦ Suc (add a b2)
  }

pub fn mul (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ Zero;
    Suc b2 ↦ add (mul a b2) a
  }

proof zero_r for add (a : Nat) : Equal Nat (add a Zero) a = Refl

pub proof zero_l for add (a : Nat) : Equal Nat (add Zero a) a =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦ cong Nat Nat (add Zero a2) a2 Suc ((proof zero_l for add) a2)
  }

proof suc_r for add (a : Nat) (b : Nat) : Equal Nat (add a (Suc b)) (Suc (add a b)) = Refl

pub proof suc_l for add (a : Nat) (b : Nat) : Equal Nat (add (Suc a) b) (Suc (add a b)) =
  match b {
    Zero ↦ Refl;
    Suc b2 ↦ cong Nat Nat (add (Suc a) b2) (Suc (add a b2)) Suc ((proof suc_l for add) a b2)
  }

pub proof assoc for add
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (add a (add b c)) (add (add a b) c) =
  match c {
    Zero ↦ Refl;
    Suc c2 ↦
      cong Nat Nat (add a (add b c2)) (add (add a b) c2) Suc ((proof assoc for add) a b c2)
  }

pub proof comm for add (a : Nat) (b : Nat) : Equal Nat (add a b) (add b a) =
  match b {
    Zero ↦ sym Nat (add Zero a) a ((proof zero_l for add) a);
    Suc b2 ↦
      trans
        Nat
        (add a (Suc b2))
        (Suc (add b2 a))
        (add (Suc b2) a)
        (cong Nat Nat (add a b2) (add b2 a) Suc ((proof comm for add) a b2))
        (sym Nat (add (Suc b2) a) (Suc (add b2 a)) ((proof suc_l for add) b2 a))
  }

proof zero_r for mul (a : Nat) : Equal Nat (mul a Zero) Zero = Proved

pub proof zero_l for mul (a : Nat) : Equal Nat (mul Zero a) Zero =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦ proof zero_l for mul a2
  }

proof suc_r for mul (a : Nat) (b : Nat) : Equal Nat (mul a (Suc b)) (add (mul a b) a) = Refl

proof suc_l for mul (a : Nat) (b : Nat) : Equal Nat (mul (Suc a) b) (add (mul a b) b) =
  match b {
    Zero ↦ Proved;
    Suc b2 ↦
      cong
        Nat
        Nat
        (add (mul (Suc a) b2) a)
        (add (add (mul a b2) a) b2)
        Suc
        (trans
          Nat
          (add (mul (Suc a) b2) a)
          (add (add (mul a b2) b2) a)
          (add (add (mul a b2) a) b2)
          (cong
            Nat
            Nat
            (mul (Suc a) b2)
            (add (mul a b2) b2)
            (λx. add x a)
            ((proof suc_l for mul) a b2))
          (trans
            Nat
            (add (add (mul a b2) b2) a)
            (add (mul a b2) (add b2 a))
            (add (add (mul a b2) a) b2)
            (sym
              Nat
              (add (mul a b2) (add b2 a))
              (add (add (mul a b2) b2) a)
              ((proof assoc for add) (mul a b2) b2 a))
            (trans
              Nat
              (add (mul a b2) (add b2 a))
              (add (mul a b2) (add a b2))
              (add (add (mul a b2) a) b2)
              (cong
                Nat
                Nat
                (add b2 a)
                (add a b2)
                (λx. add (mul a b2) x)
                ((proof comm for add) b2 a))
              ((proof assoc for add) (mul a b2) a b2))))
  }

pub proof one_r for mul (a : Nat) : Equal Nat (mul a (Suc Zero)) a = proof zero_l for add a

pub proof one_l for mul (a : Nat) : Equal Nat (mul (Suc Zero) a) a =
  match a {
    Zero ↦ Proved;
    Suc a2 ↦
      trans
        Nat
        (mul (Suc Zero) (Suc a2))
        (add a2 (Suc Zero))
        (Suc a2)
        (cong Nat Nat (mul (Suc Zero) a2) a2 (λx. add x (Suc Zero)) ((proof one_l for mul) a2))
        ((proof suc_r for add) a2 Zero)
  }

pub proof comm for mul (a : Nat) (b : Nat) : Equal Nat (mul a b) (mul b a) =
  match b {
    Zero ↦ sym Nat (mul Zero a) Zero ((proof zero_l for mul) a);
    Suc b2 ↦
      trans
        Nat
        (mul a (Suc b2))
        (add (mul b2 a) a)
        (mul (Suc b2) a)
        (cong Nat Nat (mul a b2) (mul b2 a) (λx. add x a) ((proof comm for mul) a b2))
        (sym Nat (mul (Suc b2) a) (add (mul b2 a) a) ((proof suc_l for mul) b2 a))
  }

pub theorem mul_add_distrib_r
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul a (add b c)) (add (mul a b) (mul a c)) =
  match c {
    Zero ↦ Refl;
    Suc c2 ↦
      trans
        Nat
        (add (mul a (add b c2)) a)
        (add (add (mul a b) (mul a c2)) a)
        (add (mul a b) (add (mul a c2) a))
        (cong
          Nat
          Nat
          (mul a (add b c2))
          (add (mul a b) (mul a c2))
          (λx. add x a)
          (mul_add_distrib_r a b c2))
        (sym
          Nat
          (add (mul a b) (add (mul a c2) a))
          (add (add (mul a b) (mul a c2)) a)
          ((proof assoc for add) (mul a b) (mul a c2) a))
  }

pub theorem mul_add_distrib_l
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul (add a b) c) (add (mul a c) (mul b c)) =
  trans
    Nat
    (mul (add a b) c)
    (mul c (add a b))
    (add (mul a c) (mul b c))
    ((proof comm for mul) (add a b) c)
    (trans
      Nat
      (mul c (add a b))
      (add (mul c a) (mul c b))
      (add (mul a c) (mul b c))
      (mul_add_distrib_r c a b)
      (trans
        Nat
        (add (mul c a) (mul c b))
        (add (mul a c) (mul c b))
        (add (mul a c) (mul b c))
        (cong Nat Nat (mul c a) (mul a c) (λx. add x (mul c b)) ((proof comm for mul) c a))
        (cong Nat Nat (mul c b) (mul b c) (λx. add (mul a c) x) ((proof comm for mul) c b))))

pub proof assoc for mul
      (a : Nat) (b : Nat) (c : Nat)
    : Equal Nat (mul a (mul b c)) (mul (mul a b) c) =
  match c {
    Zero ↦ Proved;
    Suc c2 ↦
      trans
        Nat
        (mul a (mul b (Suc c2)))
        (add (mul a (mul b c2)) (mul a b))
        (mul (mul a b) (Suc c2))
        (mul_add_distrib_r a (mul b c2) b)
        (cong
          Nat
          Nat
          (mul a (mul b c2))
          (mul (mul a b) c2)
          (λx. add x (mul a b))
          ((proof assoc for mul) a b c2))
  }

pub proof cancel_r for mul
      (a : Nat) (b : Nat) (k : Nat) (h : Equal Nat (mul a (Suc k)) (mul b (Suc k)))
    : Equal Nat a b =
  mul_cancel_core a b k h

theorem mul_cancel_core
      (a : Nat) (b : Nat) (k : Nat)
    : Equal Nat (mul a (Suc k)) (mul b (Suc k)) → Equal Nat a b =
  match a {
    Zero ↦
      match b {
        Zero ↦ λh. Proved;
        Suc b2 ↦
          λh.
            absurd
              (trans
                Nat
                Zero
                (mul Zero (Suc k))
                (mul (Suc b2) (Suc k))
                (sym Nat (mul Zero (Suc k)) Zero ((proof zero_l for mul) (Suc k)))
                h)
      };
    Suc a2 ↦
      match b {
        Zero ↦
          λh.
            absurd
              (trans
                Nat
                (mul (Suc a2) (Suc k))
                (mul Zero (Suc k))
                Zero
                h
                ((proof zero_l for mul) (Suc k)));
        Suc b2 ↦
          λh.
            let common_summand_equality : Equal Nat
              (add (mul a2 (Suc k)) (Suc k))
              (add (mul b2 (Suc k)) (Suc k)) =
              trans
                Nat
                (add (mul a2 (Suc k)) (Suc k))
                (mul (Suc a2) (Suc k))
                (add (mul b2 (Suc k)) (Suc k))
                (sym
                  Nat
                  (mul (Suc a2) (Suc k))
                  (add (mul a2 (Suc k)) (Suc k))
                  ((proof suc_l for mul) a2 (Suc k)))
                (trans
                  Nat
                  (mul (Suc a2) (Suc k))
                  (mul (Suc b2) (Suc k))
                  (add (mul b2 (Suc k)) (Suc k))
                  h
                  ((proof suc_l for mul) b2 (Suc k)))
            in
              cong
                Nat
                Nat
                a2
                b2
                Suc
                (mul_cancel_core
                  a2
                  b2
                  k
                  (add_cancel_r
                    (mul a2 (Suc k))
                    (mul b2 (Suc k))
                    (Suc k)
                    common_summand_equality))
      }
  }

theorem add_cancel_r
      (a : Nat) (b : Nat)
    : (c : Nat) → Equal Nat (add a c) (add b c) → Equal Nat a b =
  λc.
    match c {
      Zero ↦ λh. h;
      Suc tail ↦ λh. add_cancel_r a b tail (suc_injective (add a tail) (add b tail) h)
    }

theorem suc_injective (a : Nat) (b : Nat) (h : Equal Nat (Suc a) (Suc b)) : Equal Nat a b =
  cong Nat Nat (Suc a) (Suc b) pred_nat h

fn pred_nat (n : Nat) : Nat =
  match n {
    Zero ↦ Zero;
    Suc tail ↦ tail
  }
```

## How the document is layered

`add` and `mul` recurse on their second argument and remain the only
value-producing definitions. Their checked laws carry exactly the structural
recursion they need and are discoverable as members of `add::…` or `mul::…`.
Both distributivity laws are checked: `mul_add_distrib_r` follows the
recursion of `mul` on its right argument, while `mul_add_distrib_l`
uses commutativity to transport that result to a left-hand sum. The
multiplicative unit, zero, commutativity, and associativity laws keep their
attached `mul::…` identities. The successor multiplication helpers and the
right-zero computation proof remain private.

Cancellation by a positive right factor is also checked. Its private proof
first eliminates a common `add` summand through successor injectivity, then
recurses on both operands using the private successor multiplication law.
The exported `mul::cancel_r` needs a `Suc` factor: cancellation by `Zero`
would be false. These laws are the multiplication prerequisites for
[exact rational values](../../../../../spec/50-stdlib/64-rational.md#4-nat-prerequisite-surface).

## Using it

The operations remain ordinary values and reduce on concrete naturals.

```ken example
const add_two_three : Nat = add (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const mul_two_three : Nat = mul (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
```

## Trust derivation

The operations and attached proofs are ordinary checked
definitions. Structural recursion is on `Nat`; no trusted declaration or
numeric instance is introduced.
