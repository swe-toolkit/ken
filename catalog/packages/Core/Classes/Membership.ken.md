# `Membership` — carrier-directed Boolean membership

The provider class and ordinary binding behind the standard `∈` operator.
Providers choose a query type for one nominal carrier and compute a `Bool`.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws and observations](#4-laws-and-observations)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust and derivation](#7-trust-and-derivation)

## 1. Motivation

Membership is carrier-directed. The container identifies one canonical
provider, and that provider determines the type of queries it accepts. Keeping
the query as an associated field lets `ListMembership a`,
`OrderedKeyMembership k v`, and `RelationEdgeMembership k` remain distinct
nominal carriers even where their underlying representations overlap.

The ordinary `membership_member_at` binding is the meaning re-exported as `∈`.
It takes the dictionary explicitly so the compiler can complete its omitted
carrier and dictionary prefix through ordinary instance resolution.

## 2. Definition

`Query` precedes `member` because the operation's type depends on it. A universe
field puts the resulting structure class one universe above its carrier input:
`Membership : Type 0 → Type 1` for the catalog's current universe spelling.

```ken
import Core.Classes.LawfulClasses (IsTrue)

pub class Membership (container : Type) {
  Query : Type;
  member : Query → container → Bool
}

pub fn membership_member_at
      (c : Type) (d : Membership c) (q : d.Query) (x : c)
    : Bool =
  d.member q x

pub fn member_holds
      (c : Type) (d : Membership c) (q : d.Query) (x : c)
    : Omega =
  IsTrue (d.member q x)

pub fn same_members
      (c : Type) (d : Membership c) (x : c) (y : c)
    : Omega =
  (q : d.Query) → Eq Bool (d.member q x) (d.member q y)
```

## 3. Using it

Clients normally import a nominal view from its owning package and import `∈`
from `Core.Operators.Standard`. An explicit call may instead name
`membership_member_at`; both routes reach the same defining identity.

The dictionary fixes the query type before the query is checked. A list view
and an ordered-key view may both accept `Nat` queries, but their different
carrier heads select different providers without consulting the query.

## 4. Laws and observations

`member_holds` lifts the provider's `Bool` result into `Prop` through `IsTrue`.
It never eliminates a proposition back into runtime data.

`same_members` is observational agreement for one provider. It is a definition,
not a class field and not an equality obligation on containers. Lists with
different order or multiplicity can agree on every membership query without
being equal as lists.

Provider-specific fidelity obligations remain beside each provider because the
minimal class has no insertion, lookup, ordering, or empty-container operation
from which a nonvacuous common algebraic law could be stated.

## 5. Design notes

The class is unary. A raw `Tree` cannot be a provider because key, set, and
relation-edge membership have different meanings over that representation.
The catalog therefore uses nominal views whose values retain the comparator and
validity evidence used to construct them.

`membership_member_at` is defined exactly here. The standard-operator facade
re-exports this identity under `∈`; it does not define a second wrapper.

## 6. References

- `spec/50-stdlib/58b-membership.md` — class, providers, and law model.
- `spec/30-surface/33-declarations.md §6.3` — the ordinary standard binding.
- `spec/30-surface/39-elaboration.md §6.10` — carrier-first completion.
- `catalog/packages/Core/Classes/LawfulClasses.ken.md` — `IsTrue`.

## 7. Trust and derivation

This package introduces no `Axiom`, primitive, foreign declaration, postulate,
or unresolved hole. The class elaborates to an ordinary checked record;
`membership_member_at`, `member_holds`, and `same_members` are transparent
definitions. Its `trusted_base()` delta is zero.
