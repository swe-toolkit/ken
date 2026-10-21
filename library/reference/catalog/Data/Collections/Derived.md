# `Collections` — derived collection, string, and byte views

Availability: **partial**

Authority: **derived reference**

Canonical source: [checked package](../../../../../catalog/packages/Data/Collections/Derived.ken.md).

| Fact class | Disposition | Result and grounding |
|---|---|---|
| Subject | `generated` | `catalog/packages/Data/Collections/Derived.ken.md` — “`Collections` — derived collection, string, and byte views,” from the [subject index](../../subjects.md). |
| Declaration/type | `authored` | Declares list and natural-number combinators, verified sorting, projection-abstraction classes, derived string operations, and a structural byte-length fold. It also exports `filter_starts_after`, which preserves the sorted-head relation from a sorted list when filtering its tail, given a transitive comparator. |
| Law | `authored` | The canonical fences contain checked structural, length, sorting, projection, and round-trip-derived proofs, including list decomposition, verified `List Bool` permutation/sortedness, and the `filter_starts_after` result for sorted lists. |
| Effect/capability | `none-declared` | The canonical checked fences declare no effect row, `proc`, `visits`, or capability value for this package. |
| Assurance | `authored` | The package reports zero `trusted_base()` delta: its inductive, definitions, and proofs are checked terms, with no primitive, postulate, or law-field assumption. |
| Platform | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Maturity | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Dependency | `held` | No complete package-level checked projection exists; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Reverse dependency | `held` | No complete projection and maintained inversion exist; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
