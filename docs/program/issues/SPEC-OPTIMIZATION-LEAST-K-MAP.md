---
id: SPEC-OPTIMIZATION-LEAST-K-MAP
title: "Spec 72 selects the k least-cost candidates of a list (§4) and says how the selection behaves under a cost transform (§4.4), a larger bound (§4.5, §4.6) and an append (§4.7), but says nothing about selecting over mapped candidates: a client that ranks records by a projected key cannot move the selection across `map`. Rule from a checked D0 at zero trust that least_k over map f xs is map f of least_k over xs under the composed cost, and state it with a seed that separates the composed cost from the uncomposed one"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). §4.4 to §4.7 gave the cost, bound and append behaviour of least_k; this gives its behaviour under map. Steward-filed per COORDINATION section 2."
---

# Selecting the k least over mapped candidates

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law. For every
`a`, `b`, `f : a → b`, `cost : b → Nat`, `k` and `xs`, it concludes
`Equal (List b) (least_k b cost k (map a b f xs)) (map a b f (least_k a
(comp a b Nat cost f) k xs))`. It holds at zero trust, and spec 72 states
it by its public type.

## Settled inputs (main `f14bb6479`; anchors re-checked at release)

- **The operations.**
  - `least_k` (`Argmin.ken.md:307`) is `take a k (sort a (cost_le a cost)
    xs)`, and `cost_le` (`:305`) is `leq_nat (cost x) (cost y)`.
  - `sort` and `insert` (`Data/Collections/Derived.ken.md:1904`, `:1894`).
    `insert` places `x` before the first `h` with `le x h`.
  - `map` (`Derived.ken.md:556`) and `comp` (`Core/Function/
    Combinators.ken.md:30`, `comp a b c g h x = g (h x)`).
- **The argument, for AC-0 to check.**
  - `cost_le b cost (f x) (f y)` and `cost_le a (comp a b Nat cost f) x y`
    both reduce to `leq_nat (cost (f x)) (cost (f y))`.
  - So by induction on the list, `insert` commutes with `map f`, and then
    so does `sort`.
  - `take k` commutes with `map f` by induction on `k`.
  - Ties hold on both sides, because `map` keeps positions and `insert`
    decides on the same Boolean at each step. So `f` need not be
    injective.
- **No such law exists.** Derived has no `sort` or `take` law over `map`.
  Argmin's §4 laws (`:310` to `:547`) all keep one candidate type.
- **Imports.** Argmin already imports `comp` (`:46`). Its selective
  Derived import (`:58`) lacks `map`, and the public type needs it.
- **Spec and tests.** Spec 72 ends with §4.7
  (`72-optimization.md:1866`), so the new section is §4.8. The seed
  directory is `conformance/stdlib/optimization/`. The test pattern is
  `crates/ken-elaborator/tests/cat_optimization_least_k_append_merge.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

- Spec 72 §4.8.
- The catalog law, with any private helpers, and `map` in Argmin's
  Derived import.
- A seed and a package test.
- The status lines. In SPEC-PROGRESS row 134, which this WP touches
  anyway, also replace §4.6's pre-landing sentence ("Foundation's package
  test owes that gate. This Spec-only child ...") with its landed state.

No §1 to §4.7 operation or law type moves. There is no kernel or surface
change. Whether the `insert`, `sort` and `take` steps go into Derived as
public laws or stay private in Argmin is AC-0's ruling.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K-APPEND-MERGE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - where the step lemmas live, and whether a companion law for
    `argmin_option` belongs in this node or is not needed;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §4.8 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes `a = b = Nat`, the identity cost, `f` with `f 0
  = 2`, `f 1 = 1` and `f 2 = 0`, `xs = [0, 1, 2]` and `k = 1`.
  - Its accepted block concludes through the law that `least_k` over `map
    f xs` equals `map f` of `least_k` under the composed cost, which is
    `[0]`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    claims the same equation with the uncomposed identity cost on the
    right, which is `[2]`.
  - The two rows differ only in that cost argument.
- **AC-T.** The package test compares `trusted_base()` before and after
  Argmin as identity sets, and that comparison fails when the package
  adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The equation is false as stated, for example on tied images under a
  non-injective `f`. Stop with the counterexample; do not weaken it to a
  permutation.
- The law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.

Checks: 4 fired. I read `least_k`, `cost_le`, `sort`, `insert`, `map` and
`comp` on `f14bb6479`. The argument above uses only their definitions. 3
fired. I grepped Derived and Argmin for a `map` law over `sort`, `take` or
`least_k`, and found none. 8 fired. The refused row drops the composition,
so a law that ignores `f` in the cost fails it. Its right side is `[2]`
against the left side's `[0]`.
