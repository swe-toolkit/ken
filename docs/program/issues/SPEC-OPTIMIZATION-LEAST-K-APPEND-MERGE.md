---
id: SPEC-OPTIMIZATION-LEAST-K-APPEND-MERGE
title: "Spec 72 splits argmin, argmax and their Option forms over appended parts (§1.6, §1.8, §3.6, §3.8), but least_k has no append law, so a client cannot select the k least of a concatenation from the k least of each part. Rule from a checked D0 at zero trust that least_k cost k (xs ++ ys) equals least_k cost k (least_k cost k xs ++ least_k cost k ys), and state it with a seed that separates re-selection from appending the two part results"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Every other §72 selector has an append-split law; least_k has none. Steward-filed per COORDINATION section 2."
---

# The k least of a concatenation come from the k least of each part

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law. For every
`cost`, `k`, `xs` and `ys`, `least_k a cost k (list_append a xs ys)`
equals `least_k a cost k (list_append a (least_k a cost k xs) (least_k a
cost k ys))` as `Equal (List a)`. It holds at zero trust, and spec 72
states it by its public type.

## Settled inputs (main `831bd875f`; anchors re-checked at release)

- **The operation.** `least_k` is `take a k (sort a (cost_le a cost) xs)`
  (`Argmin.ken.md:307`; spec 72 §4.1). `sort` is stable: on a cost tie the
  earlier input occurrence comes first.
- **The vocabulary.** Argmin already imports `list_append` from
  `Data.Collections.Derived` (`Argmin.ken.md:58`). Derived exports
  `sort::perm`, `sort::stable_class`, `insert::stable_class`, the `take`
  laws (`:1227` to `:1320`) and `list_append::assoc` and `::length`.
- **The gap.** `least_k` has length, member, sorted, class, separation,
  unit, cost-order, prefix and saturation laws (§4.2 to §4.6). None
  relates a concatenation to its parts.
- **Spec and tests.** §4 ends at §4.6, so the new section is §4.7. The
  seed directory is `conformance/stdlib/optimization/`. The test pattern is
  `crates/ken-elaborator/tests/cat_optimization_least_k_saturated.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §4.7, the Argmin law with any helpers, any generic `sort` or
`take` law in Derived that the proof needs, a seed, a package test and the
status lines. No §1 to §4.6 operation or law type moves, and there is no
kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EVAL-COINCIDENCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order and the placement after
    `least_k_saturated`;
  - which supporting law is generic and belongs in Derived (for example
    a stable-sort law over `list_append`), and which stays a private
    Argmin helper;
  - every consumer root of an exported Argmin or Derived law, swept by the
    sibling `least_k_saturated` across `catalog/`, `conformance/`,
    `crates/`, `spec/` and `library/`, so the candidate's scope is
    complete.
- **AC-1.** §4.7 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to `xs = [A, B]`
  and `ys = [C, D]` with costs `2, 1, 1, 0` at `k = 2`. It concludes
  `[D, B]`, and independently checks that value. Its refused block, on a
  fresh accepted prefix and in the same form, claims instead that the
  result is `take 2` of the appended part results, `[B, A]`. A second
  accepted row at `k = 1` on `[A]` and `[C]`, both costing 1, concludes
  `[A]`: a tie across the parts keeps the earlier part.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise on
  `cost`, `k` or the inputs.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §4.6 operation or law type.

Checks: 4 fired. I read `least_k`'s body and Argmin's import of
`list_append` on `831bd875f`, so the law's type is writable in the
package's own vocabulary; the proof route is AC-0's to rule. 3 fired. I
listed §72's sections: §1.6, §1.8, §3.6 and §3.8 split the other selectors
over append, and §4 has no such law. 8 fired. On the AC-2 fixture the
refused alternative gives `[B, A]` and the law gives `[D, B]`, so the
refused row fails if the re-selection is dropped. The tie row fails if the
merge prefers the later part.
