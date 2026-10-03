```ken
theorem lookup_zip_with_scratch
      (a : Type) (b : Type) (c : Type) (n : Nat)
      (f : a → b → c) (xs : Vec a n) (ys : Vec b n) (i : Fin n)
    : Equal c
        (lookup c n (zip_with a b c n f xs ys) i)
        (f (lookup a n xs i) (lookup b n ys i)) =
  match i {
    FZero m ↦ match xs { VCons _ x tail_xs ↦ match ys {
      VCons _ y tail_ys ↦ Refl } };
    FSuc m rest ↦ match xs { VCons _ x tail_xs ↦ match ys {
      VCons _ y tail_ys ↦ lookup_zip_with_scratch a b c m f tail_xs tail_ys rest } }
  }
```
