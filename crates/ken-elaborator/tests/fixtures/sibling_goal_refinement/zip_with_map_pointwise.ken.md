```ken
import Core.Logic.Transport (sym, trans)

theorem zip_with_map_pointwise_scratch
      (a : Type) (a2 : Type) (b : Type) (b2 : Type) (c : Type) (n : Nat)
      (g : a → b) (h : a2 → b2) (f : b → b2 → c)
      (k : a → a2 → c)
      (hk : (u : a) → (v : a2) → Equal c (k u v) (f (g u) (h v)))
      (xs : Vec a n) (ys : Vec a2 n)
    : Equal (Vec c n)
        (zip_with b b2 c n f (map a b n g xs) (map a2 b2 n h ys))
        (zip_with a a2 c n k xs ys) =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦ match ys {
      VCons _ y tail_ys ↦
        trans
          (Vec c (Suc m))
          (VCons c m (f (g x) (h y))
            (zip_with b b2 c m f (map a b m g tail_xs) (map a2 b2 m h tail_ys)))
          (VCons c m (k x y)
            (zip_with b b2 c m f (map a b m g tail_xs) (map a2 b2 m h tail_ys)))
          (VCons c m (k x y) (zip_with a a2 c m k tail_xs tail_ys))
          (cong
            c (Vec c (Suc m))
            (f (g x) (h y)) (k x y)
            (λz. VCons c m z
              (zip_with b b2 c m f (map a b m g tail_xs) (map a2 b2 m h tail_ys)))
            (sym c (k x y) (f (g x) (h y)) (hk x y)))
          (cong
            (Vec c m) (Vec c (Suc m))
            (zip_with b b2 c m f (map a b m g tail_xs) (map a2 b2 m h tail_ys))
            (zip_with a a2 c m k tail_xs tail_ys)
            (VCons c m (k x y))
            (zip_with_map_pointwise_scratch a a2 b b2 c m g h f k hk tail_xs tail_ys))
    }
  }
```
