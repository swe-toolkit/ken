# f6 reconstructed: original source unavailable, not an exact-source pin.
# Consumes a named Nat equality premise through two nested sibling splits.

```ken
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}

data Fin : Nat → Type where {
  FZero : (n : Nat) → Fin (Suc n);
  FSuc : (n : Nat) → Fin n → Fin (Suc n)
}

theorem t6_reconstructed (a : Type) (b : Type) (n : Nat)
  (xs : Vec a n) (ys : Vec b n) (i : Fin n)
  (d : Equal Nat n n) : Equal Nat n n =
  match i {
    FZero m ↦ match xs { VCons _ x tail_xs ↦ match ys {
      VCons _ y tail_ys ↦ let witness = d in Refl } };
    FSuc m rest ↦ match xs { VCons _ x tail_xs ↦ match ys {
      VCons _ y tail_ys ↦ let witness = d in Refl } }
  }
```
