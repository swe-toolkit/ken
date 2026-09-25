# `Not` — propositional negation

`Not p` is a function from evidence of `p` to the proposition `Bottom`.
It remains a proposition: neither `Bottom` nor its negation introduces a
computational decision about `p`.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws and proofs](#4-laws-and-proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust and derivation](#7-trust-and-derivation)

## 1. Motivation

`Bottom` is falsehood among propositions. When an assumption `p` entails
`Bottom`, its contradiction can be named and passed to another proof. This is
different from `Empty`, the Type-sorted uninhabited carrier used by a decision
procedure's refutation branch.

## 2. Definition

The proposition is an ordinary function type. `Bottom` is the checked
propositional falsehood, not a new primitive or postulate.

```ken
fn Not (p : Omega) : Omega = p → Bottom

export Not
```

## 3. Using it

Given `refute : Not p`, applying `refute` to evidence of `p` produces a proof of
`Bottom`; its elimination then discharges a proposition. For inspectable
positive-or-negative evidence use `Dec` from `Core.Logic.EmptyDec` instead.

## 4. Laws and proofs

There is no separate law: `Not p` reduces to `p → Bottom` by definition.
Any proof using this reduction is checked at its own proposition.

## 5. Design notes

`Not` does not inspect an argument or choose a branch. In particular it is not
a replacement for `No`'s function into the Type-sorted `Empty`.

## 6. References

- [Negation](https://en.wikipedia.org/wiki/Negation) — propositional
  negation as contradiction.

## 7. Trust and derivation

The public API is `Not`. Its body forms a function type from the existing
propositional falsehood. It adds no axiom, postulate, primitive, or trusted-base
entry. Consumers import `Core.Logic.Not (Not)`.
