# `Or` — proof-relevant disjunction

`Or` packages evidence for one of two propositions while retaining which side
holds. Its parameters live in `Omega`, but the family itself lives in `Type`,
so consumers may distinguish `Inl` from `Inr` by case analysis.

## Definition

The family is ordinary checked data. Its public interface exports the family
and both constructors from this module.

```ken
data Or (a : Omega) (b : Omega) : Type where {
  Inl : a → Or a b;
  Inr : b → Or a b
}

export Or, Inl, Inr
```

`Inl` carries evidence for the left proposition, and `Inr` carries evidence
for the right proposition. The result remains `Type`-sorted deliberately:
placing the two-constructor family in `Omega` would make its inhabitants
proof-irrelevant and erase the branch tag.

## Trust and derivation

`Or` elaborates through the ordinary explicit-data path and is re-checked by
the kernel as an inductive family. It adds no postulate, primitive, kernel
rule, or trusted-base entry. Modules and exports preserve the declaration's
single `GlobalId` while elaborating away to the same flat kernel environment.
