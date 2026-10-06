# Length-indexed vectors

`Vec a n` records its length in its type, making non-empty and in-bounds
operations total by construction.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

A list does not state its length in its type, so `head` and positional lookup
must account for empty or out-of-bounds inputs. A vector carries a `Nat` index
that describes its length. `Vec a (Suc n)` therefore excludes the empty
constructor, and `Fin n` represents only positions that are strictly below
`n`.

The same index also states the alignment contract of `map` and `zip_with`.
Their result types retain the input length, and `zip_with` accepts only two
vectors with the same length.

## Definition

`Vec` and `Fin` are ordinary indexed inductive families. `VNil` targets length
`Zero`; `VCons` extends a vector at length `n` to length `Suc n`. Neither
constructor of `Fin` targets `Fin Zero`. The map laws use checked `idf` and
`comp` from the combinator package and `cong` from the transport package.

```ken
import Core.Function.Combinators (comp, idf)

import Core.Logic.Transport (cong, sym, trans)

import Data.Collections.Derived (length)

data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}

data Fin : Nat → Type where {
  FZero : (n : Nat) → Fin (Suc n);
  FSuc : (n : Nat) → Fin n → Fin (Suc n)
}

fn head (a : Type) (n : Nat) (xs : Vec a (Suc n)) : a =
  match xs {
    VCons m x tail_xs ↦ x
  }

fn tail (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Vec a n =
  match xs {
    VCons m x tail_xs ↦ tail_xs
  }

fn map (a : Type) (b : Type) (n : Nat) (f : a → b) (xs : Vec a n) : Vec b n =
  match xs {
    VNil ↦ VNil b;
    VCons m x tail_xs ↦ VCons b m (f x) (map a b m f tail_xs)
  }

fn zip_with
      (a : Type) (b : Type) (c : Type) (n : Nat) (f : a → b → c) (xs : Vec a n) (ys : Vec b n)
    : Vec c n =
  match xs {
    VNil ↦ VNil c;
    VCons m x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦ VCons c m (f x y) (zip_with a b c m f tail_xs tail_ys)
      }
  }

pub fn to_list (a : Type) (n : Nat) (xs : Vec a n) : List a =
  match xs {
    VNil ↦ Nil a;
    VCons m x tail_xs ↦ Cons a x (to_list a m tail_xs)
  }

pub fn zip (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n) : Vec (Pair a b) n =
  zip_with a b (Pair a b) n (mk_pair a b) xs ys

pub fn unzip (a : Type) (b : Type) (n : Nat) (ps : Vec (Pair a b) n)
    : Pair (Vec a n) (Vec b n) =
  match ps {
    VNil ↦ mk_pair (Vec a Zero) (Vec b Zero) (VNil a) (VNil b);
    VCons m p tail_ps ↦
      let tails = unzip a b m tail_ps in
        mk_pair
          (Vec a (Suc m))
          (Vec b (Suc m))
          (VCons a m (pair_fst a b p) (pair_fst (Vec a m) (Vec b m) tails))
          (VCons b m (pair_snd a b p) (pair_snd (Vec a m) (Vec b m) tails))
  }

fn lookup (a : Type) (n : Nat) (xs : Vec a n) (i : Fin n) : a =
  match i {
    FZero m ↦
      match xs {
        VCons _ x tail_xs ↦ x
      };
    FSuc m rest ↦
      match xs {
        VCons _ x tail_xs ↦ lookup a m tail_xs rest
      }
  }

theorem to_list_length
      (a : Type) (n : Nat) (xs : Vec a n)
    : Equal Nat (length a (to_list a n xs)) n =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦
      cong Nat Nat (length a (to_list a m tail_xs)) m Suc (to_list_length a m tail_xs)
  }

theorem unzip_zip_fst
      (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n)
    : Equal
        (Vec a n)
        (pair_fst (Vec a n) (Vec b n) (unzip a b n (zip a b n xs ys)))
        xs =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          cong
            (Vec a m)
            (Vec a (Suc m))
            (pair_fst (Vec a m) (Vec b m) (unzip a b m (zip a b m tail_xs tail_ys)))
            tail_xs
            (VCons a m x)
            (unzip_zip_fst a b m tail_xs tail_ys)
      }
  }

theorem zip_unzip
      (a : Type) (b : Type) (n : Nat) (ps : Vec (Pair a b) n)
    : Equal
        (Vec (Pair a b) n)
        (zip
          a
          b
          n
          (pair_fst (Vec a n) (Vec b n) (unzip a b n ps))
          (pair_snd (Vec a n) (Vec b n) (unzip a b n ps)))
        ps =
  match ps {
    VNil ↦ Proved;
    VCons m p tail_ps ↦
      cong
        (Vec (Pair a b) m)
        (Vec (Pair a b) (Suc m))
        (zip
          a
          b
          m
          (pair_fst (Vec a m) (Vec b m) (unzip a b m tail_ps))
          (pair_snd (Vec a m) (Vec b m) (unzip a b m tail_ps)))
        tail_ps
        (VCons (Pair a b) m p)
        (zip_unzip a b m tail_ps)
  }

theorem head_vcons
      (a : Type) (n : Nat) (x : a) (xs : Vec a n)
    : Equal a (head a n (VCons a n x xs)) x =
  Refl

theorem tail_vcons
      (a : Type) (n : Nat) (x : a) (xs : Vec a n)
    : Equal (Vec a n) (tail a n (VCons a n x xs)) xs =
  Refl

theorem map_vnil
      (a : Type) (b : Type) (f : a → b)
    : Equal (Vec b Zero) (map a b Zero f (VNil a)) (VNil b) =
  Proved

theorem vec_map_identity
      (a : Type) (n : Nat) (xs : Vec a n)
    : Equal (Vec a n) (map a a n (idf a) xs) xs =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦
      cong
        (Vec a m)
        (Vec a (Suc m))
        (map a a m (idf a) tail_xs)
        tail_xs
        (VCons a m x)
        (vec_map_identity a m tail_xs)
  }

theorem zip_with_vnil
      (a : Type) (b : Type) (c : Type) (f : a → b → c)
    : Equal (Vec c Zero) (zip_with a b c Zero f (VNil a) (VNil b)) (VNil c) =
  Proved

theorem zip_with_vcons
      (a : Type)
      (b : Type)
      (c : Type)
      (n : Nat)
      (f : a → b → c)
      (x : a)
      (xs : Vec a n)
      (y : b)
      (ys : Vec b n)
    : Equal
        (Vec c (Suc n))
        (zip_with a b c (Suc n) f (VCons a n x xs) (VCons b n y ys))
        (VCons c n (f x y) (zip_with a b c n f xs ys)) =
  Refl

theorem lookup_fzero
      (a : Type) (n : Nat) (x : a) (xs : Vec a n)
    : Equal a (lookup a (Suc n) (VCons a n x xs) (FZero n)) x =
  Refl

theorem lookup_fsuc
      (a : Type) (n : Nat) (x : a) (xs : Vec a n) (i : Fin n)
    : Equal a (lookup a (Suc n) (VCons a n x xs) (FSuc n i)) (lookup a n xs i) =
  Refl

theorem map_vcons
      (a : Type) (b : Type) (n : Nat) (f : a → b) (x : a) (xs : Vec a n)
    : Equal
        (Vec b (Suc n))
        (map a b (Suc n) f (VCons a n x xs))
        (VCons b n (f x) (map a b n f xs)) =
  Refl

theorem vec_map_compose
      (a : Type) (b : Type) (c : Type) (n : Nat) (f : a → b) (g : b → c) (xs : Vec a n)
    : Equal (Vec c n) (map b c n g (map a b n f xs)) (map a c n (comp a b c g f) xs) =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦
      cong
        (Vec c m)
        (Vec c (Suc m))
        (map b c m g (map a b m f tail_xs))
        (map a c m (comp a b c g f) tail_xs)
        (VCons c m (g (f x)))
        (vec_map_compose a b c m f g tail_xs)
  }

theorem lookup_map
      (a : Type) (b : Type) (n : Nat) (f : a → b) (xs : Vec a n) (i : Fin n)
    : Equal b (lookup b n (map a b n f xs) i) (f (lookup a n xs i)) =
  match i {
    FZero m ↦
      match xs {
        VCons _ x tail_xs ↦ Refl
      };
    FSuc m rest ↦
      match xs {
        VCons _ x tail_xs ↦ lookup_map a b m f tail_xs rest
      }
  }

theorem lookup_zip_with
      (a : Type)
      (b : Type)
      (c : Type)
      (n : Nat)
      (f : a → b → c)
      (xs : Vec a n)
      (ys : Vec b n)
      (i : Fin n)
    : Equal c
        (lookup c n (zip_with a b c n f xs ys) i)
        (f (lookup a n xs i) (lookup b n ys i)) =
  match i {
    FZero m ↦
      match xs {
        VCons _ x tail_xs ↦
          match ys {
            VCons _ y tail_ys ↦ Refl
          }
      };
    FSuc m rest ↦
      match xs {
        VCons _ x tail_xs ↦
          match ys {
            VCons _ y tail_ys ↦ lookup_zip_with a b c m f tail_xs tail_ys rest
          }
      }
  }

theorem zip_with_map
      (a : Type)
      (a2 : Type)
      (b : Type)
      (b2 : Type)
      (c : Type)
      (n : Nat)
      (g : a → b)
      (h : a2 → b2)
      (f : b → b2 → c)
      (k : a → a2 → c)
      (hk : (u : a) → (v : a2) → Equal c (k u v) (f (g u) (h v)))
      (xs : Vec a n)
      (ys : Vec a2 n)
    : Equal
        (Vec c n)
        (zip_with b b2 c n f (map a b n g xs) (map a2 b2 n h ys))
        (zip_with a a2 c n k xs ys) =
  match xs {
    VNil ↦ Proved;
    VCons m x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          let
            mapped_tail = zip_with b b2 c m f (map a b m g tail_xs) (map a2 b2 m h tail_ys);
            original_tail = zip_with a a2 c m k tail_xs tail_ys;
            head_alignment = sym c (k x y) (f (g x) (h y)) (hk x y);
            tail_alignment = zip_with_map a a2 b b2 c m g h f k hk tail_xs tail_ys
          in
            trans
              (Vec c (Suc m))
              (VCons c m (f (g x) (h y)) mapped_tail)
              (VCons c m (k x y) mapped_tail)
              (VCons c m (k x y) original_tail)
              (cong
                c
                (Vec c (Suc m))
                (f (g x) (h y))
                (k x y)
                (λz. VCons c m z mapped_tail)
                head_alignment)
              (cong
                (Vec c m)
                (Vec c (Suc m))
                mapped_tail
                original_tail
                (VCons c m (k x y))
                tail_alignment)
      }
  }
```

## Using it

`VNil Bool` has type `Vec Bool Zero`. Applying `VCons Bool Zero True` to it
produces a `Vec Bool (Suc Zero)`, which is accepted by `head` and `tail`.
There is no corresponding call of `head` on `VNil`: its type cannot satisfy the
required successor-length index.

`FZero n` selects the first element of a vector of length `Suc n`. `FSuc n i`
selects the position after `i`; its constructor requires `i : Fin n`, so each
recursive lookup step consumes one vector element and one bound witness
together.

`map` changes only the element type. `zip_with` requires both inputs at the
same `n` and returns its output at that same `n`, so truncation cannot occur.

## Laws & proofs

Length preservation is carried by the signatures:

- `map` returns `Vec b n` from `Vec a n`.
- `zip_with` returns `Vec c n` from two inputs at the same `n`.

No separate arithmetic theorem is needed to recover those facts. The kernel
checks the index at every constructor assembly and recursive call.

Totality is likewise carried by the domain types. `head` and `tail` accept only
`Vec a (Suc n)`, while `lookup` requires a `Fin n` paired with `Vec a n`.
Impossible empty branches are omitted only where the index refutes them; the
elaborator still supplies a total dependent eliminator to the kernel.

Eight computation theorems are checked proof terms. The cons and bounded-index
cases reduce to reflexive equalities and close with `Refl`, including the
generic cons case of `zip_with` over both vector tails. The empty `map` and
`zip_with` results reduce to the same nullary constructor, so their equalities
collapse and close with `Proved`.

Mapping `idf a` over any vector returns the same vector. Composition of two
maps equals mapping their composite `comp a b c g f`. The two empty cases
collapse; in each successor case, `cong` lifts the recursive equality under
`VCons`. Looking up an element after mapping is the same as mapping the
original lookup result: matching `Fin n`, then its vector, follows the index
into the successor tail. To look up after `zip_with`, match the bounded index
first and then peel both vectors at that index. The first-position case reduces
reflexively; the successor case reuses the same property for both tails. These
laws are private checked proofs, not exports.

Pointwise alignment of `k` with the mapped binary function makes mapping
before `zip_with` equivalent to zipping with `k`. In the successor case,
`sym` orients the head alignment toward `k`, while `cong` lifts that equality
and the recursive tail equality under `VCons`; `trans` joins the two changes.
This checked private theorem works at arbitrary element types and lengths.

The checked examples use the private computation and lookup laws at their
generic propositions, then illustrate the operations at concrete indices.
The Boolean helpers exist only for those illustrations and are not package laws.

```ken example
fn vec_example_not (x : Bool) : Bool =
  match x {
    True ↦ False;
    False ↦ True
  }

fn vec_example_and (x : Bool) (y : Bool) : Bool =
  match x {
    True ↦ y;
    False ↦ False
  }

theorem use_lookup_fsuc
      (a : Type) (n : Nat) (x : a) (xs : Vec a n) (i : Fin n)
    : Equal a (lookup a (Suc n) (VCons a n x xs) (FSuc n i)) (lookup a n xs i) =
  lookup_fsuc a n x xs i

theorem use_map_vcons
      (a : Type) (b : Type) (n : Nat) (f : a → b) (x : a) (xs : Vec a n)
    : Equal
        (Vec b (Suc n))
        (map a b (Suc n) f (VCons a n x xs))
        (VCons b n (f x) (map a b n f xs)) =
  map_vcons a b n f x xs

theorem use_zip_with_vcons
      (a : Type)
      (b : Type)
      (c : Type)
      (n : Nat)
      (f : a → b → c)
      (x : a)
      (xs : Vec a n)
      (y : b)
      (ys : Vec b n)
    : Equal
        (Vec c (Suc n))
        (zip_with a b c (Suc n) f (VCons a n x xs) (VCons b n y ys))
        (VCons c n (f x y) (zip_with a b c n f xs ys)) =
  zip_with_vcons a b c n f x xs y ys

theorem use_vec_map_compose
      (a : Type) (b : Type) (c : Type) (n : Nat) (f : a → b) (g : b → c) (xs : Vec a n)
    : Equal (Vec c n) (map b c n g (map a b n f xs)) (map a c n (comp a b c g f) xs) =
  vec_map_compose a b c n f g xs

theorem use_lookup_map
      (a : Type) (b : Type) (n : Nat) (f : a → b) (xs : Vec a n) (i : Fin n)
    : Equal b (lookup b n (map a b n f xs) i) (f (lookup a n xs i)) =
  lookup_map a b n f xs i

theorem vec_example_lookup_second
    : Equal Bool
        (lookup
          Bool
          (Suc (Suc Zero))
          (VCons Bool (Suc Zero) True (VCons Bool Zero False (VNil Bool)))
          (FSuc (Suc Zero) (FZero Zero)))
        False =
  Proved

theorem vec_example_map_second
    : Equal Bool
        (lookup
          Bool
          (Suc (Suc Zero))
          (map
            Bool
            Bool
            (Suc (Suc Zero))
            vec_example_not
            (VCons Bool (Suc Zero) True (VCons Bool Zero False (VNil Bool))))
          (FSuc (Suc Zero) (FZero Zero)))
        True =
  Proved

theorem vec_example_zip_second
    : Equal Bool
        (lookup
          Bool
          (Suc (Suc Zero))
          (zip_with
            Bool
            Bool
            Bool
            (Suc (Suc Zero))
            vec_example_and
            (VCons Bool (Suc Zero) False (VCons Bool Zero True (VNil Bool)))
            (VCons Bool (Suc Zero) True (VCons Bool Zero False (VNil Bool))))
          (FSuc (Suc Zero) (FZero Zero)))
        False =
  Proved
```

## Design notes

`Fin` is preferred to an unrestricted `Nat` plus a separate less-than proof.
Its constructors make the bound structural and give the accessor a single,
canonical totality story.

Constructor names are PascalCase because constructors are type-like public
names on the current surface. Function names are snake_case; in particular,
`zip_with` follows the catalog naming convention while preserving the usual
zip-with operation.

The implementation recurses structurally. `zip_with` and `lookup` refine a
sibling indexed value through nested matches. Generic cons computation for
`zip_with` checks by `Refl`. Lookup after `zip_with` follows its bounded index
through both input tails and is checked generically. `zip_with`/map naturality
states a pointwise premise on the combining function; this states the general
law without an inline lambda in a proposition type. Concrete checked
examples illustrate the operations but do not stand in for those general laws.

## References

- [Dependent type](https://en.wikipedia.org/wiki/Dependent_type) — an overview
  of types that depend on values and the role of indexed families.
- Ulf Norell, *Dependently Typed Programming in Agda* — Chalmers University of
  Technology, 2007 — introduces vectors and bounded indices as standard
  dependent-programming examples.
- Daniel P. Friedman and David Thrane Christiansen, *The Little Typer*, MIT
  Press, 2018 — a book-length introduction to programming with dependent
  types.

## Trust & derivation

This entry realizes the length-indexed vector contract in
`spec/50-stdlib/60-length-indexed-vectors.md` using the ordinary `Nat`, indexed
`data`, structural recursion, dependent `match`, `Equal`, `Refl`, and `Proved`
surfaces. The private map and naturality laws reuse
`Core.Function.Combinators.comp`/`idf` and
`Core.Logic.Transport.cong`/`sym`/`trans`.

The public API is `Vec`, `VNil`, `VCons`, `Fin`, `FZero`, `FSuc`, `head`,
`tail`, `map`, `zip_with`, and `lookup`. Eight computation theorems, map
composition, lookup after map and after `zip_with`, and pointwise
`zip_with`/map naturality are private checked laws.

`Vec` and `Fin` are kernel-checked inductive families. Every function is a
transparent definition, every theorem has a checked proof term, and the entry
adds no axiom, postulate, primitive, foreign declaration, or unresolved hole.
Its cold roots-loaded `trusted_base()` set equals a separately fresh compiler
base set. The imported combinator and congruence providers contribute no
trusted items, and Vector adds none.

Targeted validation checks the package through the roots-based module loader,
the exact family indices and constructor targets, generic operation types,
rejection of empty and out-of-bounds calls, computation and identity theorems,
and the provider-closure before/after trusted-base set.
