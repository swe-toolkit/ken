# `And` — proof-relevant conjunction

`And` retains evidence for two propositions in a `Type`-sorted value.
Consumers may inspect its two proofs before truncating the conjunction into
`Omega`.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A nondeterministic path step needs evidence both for an edge and for the
remainder of the path. `And` packages these proofs without asking the general
`Pair` carrier to accept `Omega`-sorted fields. The result is a reusable
conjunction alongside `Core.Logic.Or`.

## 2. Definition

The family is `Type`-sorted, even though its two parameters are propositions.

```ken
pub data And (left : Omega) (right : Omega) : Type where {
  Both : left → right → And left right
}

export Both
```

## 3. Using it

Supply both proofs to construct the conjunction. Truncate the result when
only the proposition that both facts hold matters.

```ken example
const both_true : And (Equal Bool True True) (Equal Bool True True) =
  Both (Equal Bool True True) (Equal Bool True True) Proved Proved
```

## 4. Laws & proofs

`Both` requires one checked proof of each argument. Case analysis on an `And`
value recovers both proofs; no postulate or separate conjunction law is needed.

## 5. Design notes

Keeping `And` in `Type` preserves its evidence for pattern matching. An
NFA path uses `‖ And ... ... ‖` to make the corresponding proposition
proof-irrelevant without changing the reusable conjunction's carrier.

## 6. References

- [Conjunction](https://en.wikipedia.org/wiki/Logical_conjunction) —
  Wikipedia; orientation to the logical operation represented by this family.
- [Formal languages §3](../../../../spec/50-stdlib/61-formal-languages.md) —
  the checked path-acceptance contract that consumes proof-relevant conjunction.

## 7. Trust & derivation

The public names are `And` and `Both`; the definition is the entire checked
inductive family. The derivation uses the existing `Omega`, `Type`, and checked
inductive rules. It adds no local `trusted_base()` entry, primitive, Axiom, or
postulate. An NFA path consumer imports this conjunction explicitly.
