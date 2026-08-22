# Verified Euclidean greatest common divisor

This package presents Euclid's algorithm over unary natural numbers. The
algorithm uses subtraction, while an explicit structural fuel argument makes
the decrease visible to Ken's termination checker.

## Definition

The arithmetic operations are local structural definitions. `Divides` carries
an explicit quotient and a checked multiplication equation.

```ken
fn add (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ a;
    Suc b2 ↦ Suc (add a b2)
  }

fn mul (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ Zero;
    Suc b2 ↦ add (mul a b2) a
  }

fn leq_nat (a : Nat) (b : Nat) : Bool =
  match a {
    Zero ↦ True;
    Suc a2 ↦
      match b {
        Zero ↦ False;
        Suc b2 ↦ leq_nat a2 b2
      }
  }

fn sub (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ a;
    Suc b2 ↦
      match a {
        Zero ↦ Zero;
        Suc a2 ↦ sub a2 b2
      }
  }

data Divides (d : Nat) (n : Nat) : Type where {
  MkDivides :
    (quotient : Nat)
    → Equal Nat n (mul d quotient)
    → Divides d n
}

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

fn gcd (a : Nat) (b : Nat) : Nat = gcd_fuel (Suc (add a b)) a b
```

## Laws and proofs

The proof layer establishes the fuel invariant and then specializes it to the
public fuel bound.

```ken
```

## Trust and derivation

`gcd_fuel` is structural on its explicit fuel. The public fuel is one more than
the sum of the inputs; every positive subtraction step strictly lowers that
sum. `Divides` is witness-bearing data, and the required laws are checked proof
terms. This package introduces no axiom, postulate, primitive, or foreign
declaration.
