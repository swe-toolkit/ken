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
constructor of `Fin` targets `Fin Zero`.

```ken
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

theorem zip_with_vnil
      (a : Type) (b : Type) (c : Type) (f : a → b → c)
    : Equal (Vec c Zero) (zip_with a b c Zero f (VNil a) (VNil b)) (VNil c) =
  Proved

theorem lookup_fzero
      (a : Type) (n : Nat) (x : a) (xs : Vec a n)
    : Equal a (lookup a (Suc n) (VCons a n x xs) (FZero n)) x =
  Refl
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

The five computation theorems are checked proof terms. The successor-vector
and first-index cases reduce to reflexive equalities and close with `Refl`.
The empty `map` and `zip_with` results reduce to the same nullary constructor,
so their equalities collapse and close with `Proved`.

## Design notes

`Fin` is preferred to an unrestricted `Nat` plus a separate less-than proof.
Its constructors make the bound structural and give the accessor a single,
canonical totality story.

Constructor names are PascalCase because constructors are type-like public
names on the current surface. Function names are snake_case; in particular,
`zip_with` follows the catalog naming convention while preserving the usual
zip-with operation.

The implementation recurses structurally. `zip_with` and `lookup` refine a
sibling indexed value through a nested match, so their recursive steps consume
only tails whose indices have been refined to the same predecessor.

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
surfaces.

The public API is `Vec`, `VNil`, `VCons`, `Fin`, `FZero`, `FSuc`, `head`,
`tail`, `map`, `zip_with`, and `lookup`, together with the five computation
theorems above.

`Vec` and `Fin` are kernel-checked inductive families. Every function is a
transparent definition, every theorem has a checked proof term, and the entry
adds no axiom, postulate, primitive, foreign declaration, or unresolved hole.
Its `trusted_base()` delta is zero.

Targeted validation checks the package through the roots-based module loader,
the exact family indices and constructor targets, generic operation types,
rejection of empty and out-of-bounds calls, computation theorems, and the
before/after trusted-base set.
