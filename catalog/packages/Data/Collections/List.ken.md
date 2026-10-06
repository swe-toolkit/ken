# List length

The canonical structural length of a `List` lives below derived collection,
ordering, and text operations, so a client can use it without inheriting their
unrelated assumptions.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

Length is a fold on the built-in inductive `List`. Its computation requires
only the built-in `Nat` constructors; it needs no order, text conversion,
class instance, or catalog import.

## Definition

```ken
pub fn length (a : Type) (xs : List a) : Nat =
  match xs {
    Nil ↦ Zero;
    Cons h t ↦ Suc (length a t)
  }
```

## Using it

Import `length` from `Data.Collections.List` when only list size is needed.
`Data.Collections.Derived` also re-exports the same checked identity for its
existing consumers.

```ken example
import Data.Collections.List (length)

const two_elements : Nat = length Bool (Cons Bool True (Cons Bool False (Nil Bool)))
```

## Laws & proofs

Both constructor equations follow by reducing the transparent definition.
`Data.Collections.Derived` proves the attached append-length law and the map,
take, drop, and indexed-lookup laws using this same definition.

## Design notes

The recursion consumes exactly one `Cons` tail at each step. Keeping this
operation in a module with no catalog imports lets a client reuse the canonical
fold without loading the richer derived-collections dependency closure.

## References

None. This is a direct structural fold over Ken's built-in `List`.

## Trust & derivation

The public API is the transparent, structurally recursive `length`. Its cold
roots-loaded trusted-base set equals a separately fresh compiler base set.
This module declares no axiom, primitive, foreign value, or additional
trusted entry. The longer list laws remain in `Data.Collections.Derived`,
which re-exports this definition without creating a second identity.
