# Function combinators

The identity and composition functions used by constructor-class laws have a
small, independently loadable provider. Neither definition assumes a class
instance or an imported law.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

A proof about a simple function need not import the algebra, collection, or
text packages that happen to use that function in their own laws.

## Definition

`idf` returns its argument. `comp` applies its second function, then its
first.

```ken
pub fn idf (a : Type) (x : a) : a = x

pub fn comp (a : Type) (b : Type) (c : Type) (g : b → c) (h : a → b) (x : a) : c = g (h x)
```

## Using it

Apply `idf a` wherever a function of type `a → a` is needed. Apply
`comp a b c g h` to obtain a function of type `a → c`.

## Laws & proofs

Both bodies are transparent definitions. Applications reduce directly to
`x` and `g (h x)` respectively. This entry adds no separate law declaration.

## Design notes

This provider imports no other catalog package. The constructor-class laws in
`Core.Classes.LawfulFunctors` use these same two canonical functions; clients
that need either function import it directly from here.

## References

None. These are elementary functions with no external implementation dependency.

## Trust & derivation

The public API is `idf` and `comp`. Both are transparent, checked pure
functions. The cold provider closure adds no trusted declaration to a fresh
compiler environment. No axiom, postulate, primitive, or foreign declaration
is introduced. Targeted roots-loader checks compare trusted-base sets and
verify the canonical imported identities in consuming proofs.
