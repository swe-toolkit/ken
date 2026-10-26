# `And` — conjunction of propositions

`And a b` pairs proofs of two propositions without carrying a relevant
computational witness.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws and proofs](#4-laws-and-proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust and derivation](#7-trust-and-derivation)

## 1. Motivation

When both `a` and `b` are propositions, evidence for their conjunction
consists of a proof of each. The pair remains in `Omega`: it does not reveal a
relevant value at runtime. This package provides the proposition-sorted form
used by checked collection predicates and other proof obligations.

## 2. Definition

The expression-position dependent pair forms the conjunction directly. The
public `And` spelling republishes the same checked definition as `and`; it
is not a second definition.

```ken
pub fn and (a : Omega) (b : Omega) : Omega = (x : a) × b

export and as And
```

## 3. Using it

Selectively import `And` and the needed introduction or projection theorem
from `Core.Logic.Conjunction`. `and_intro a b x y` builds evidence of both
propositions; `and_fst a b p` and `and_snd a b p` recover the corresponding
proofs. The constituent propositions can depend on values such as adjacent
list elements without making the conjunction proof-relevant.

## 4. Laws and proofs

The constructor and projections are ordinary checked proof terms for the
proposition-sorted pair. No separate assumption is needed to introduce or
eliminate either conjunct.

```ken
pub theorem and_intro (a : Omega) (b : Omega) (x : a) (y : b) : and a b = (x, y)

pub theorem and_fst (a : Omega) (b : Omega) (p : and a b) : a = p.1

pub theorem and_snd (a : Omega) (b : Omega) (p : and a b) : b = p.2
```

## 5. Design notes

`Core.Logic.And` is a distinct Type-sorted, proof-relevant carrier with its
own constructor. Its `And` is not this proposition-sorted `And`; clients
choose the appropriate module explicitly. Both conjuncts here are in `Omega`,
so the pair also remains in `Omega` and its proofs are irrelevant.

## 6. References

- [Logical conjunction](https://en.wikipedia.org/wiki/Logical_conjunction)
  — orientation for the proposition and its introduction and eliminations.

## 7. Trust and derivation

The public API consists of `And` (the export of `and`), `and_intro`,
`and_fst` and `and_snd`. The definition uses the existing proposition-sorted
pair, and its three proof terms use pair introduction and projections. All
four declarations are transparent and checked; this package introduces no
axiom, primitive, postulate or trusted-base entry. The compiler's prelude
retains a separately installed conjunction until its own removal; only the
catalog import selects the public package identity for a client.
