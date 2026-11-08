# Core.Effect.Coproduct

`Core.Effect.Coproduct` publishes the checked inclusions that lift an
interaction tree into either side of an effect coproduct.

## Contents

[Motivation](#motivation), [Definition](#definition), [Using it](#using-it),
[Laws & proofs](#laws--proofs), [Design notes](#design-notes),
[References](#references), [Trust & derivation](#trust--derivation).

## Motivation

An `ITree` over one effect signature can be placed in a larger tree over the
coproduct of two signatures. The checked left and right inclusions keep the
original response family of the selected side. They do not grant a capability
or perform an operation by themselves.

## Definition

```ken
export inject_l, inject_r
```

## Using it

Import both inclusions when constructing a tree over a composed effect
signature. Their checked types are:

```text
inject_l : (g h : Type) -> (rg : g -> Type) -> (rh : h -> Type) -> (a : Type) ->
  ITree g rg a -> ITree (Coproduct g h) (resp_coproduct g h rg rh) a
inject_r : (g h : Type) -> (rg : g -> Type) -> (rh : h -> Type) -> (a : Type) ->
  ITree h rh a -> ITree (Coproduct g h) (resp_coproduct g h rg rh) a
```

`Coproduct`, `ITree`, and `resp_coproduct` are separate checked floor
vocabulary. The inclusions remain ordinary checked definitions, available only
through this package's explicit import.

## Laws & proofs

This entry states no new laws or proofs. The two inclusions retain the checked
definitions and types registered before package source is elaborated.

## Design notes

These are general inclusions for arbitrary effect signatures. The
`Capability.Process.Exit` host helpers lift specific Console and filesystem
operations into `HostIO`; they are not a substitute for a direct composed
effect tree.

## References

[Ken's effect and handler contract](../../../../spec/30-surface/36-effects.md)
sets the coproduct response and inclusion signatures.

## Trust & derivation

`inject_l` and `inject_r` are existing, checked structural `elim_ITree`
re-tagging definitions. This module only exports their original checked
identities; it adds no kernel primitive, postulate, axiom, or trusted-base
entry. Any effects performed by an included tree remain governed by the
caller's declared row and capability boundary.
