---
id: SPEC-OPTIMIZATION-LEAST-K-COST-ORDER-INVARIANCE
title: "Spec 72 states cost-order invariance for argmin, argmin_option, argmax and argmax_option but not for least_k, so a client cannot rescale a top-k cost by a strictly increasing transform and keep the selection. Rule from a checked D0 at zero trust that least_k under comp f cost equals least_k under cost, on the same strict-order premise as §1.5, and state it with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md); every spec 72 selector except least_k has an invariance law (§1.5, §1.7, §3.5, §3.7). Steward-filed per COORDINATION section 2."
---

# Rescaling the cost keeps the k cheapest

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law that, for
any `f : Nat → Nat` which preserves and reflects `lt_nat`,
`least_k a (comp a Nat Nat f cost) k xs` equals `least_k a cost k xs`
as `Equal (List a)`. It holds at zero trust, and spec 72 states it by
its public type.

## Settled inputs (main `b4049048f`)

- **The gap.** §4 (`spec/50-stdlib/72-optimization.md:1400`) states
  eight `least_k` laws and no invariance law. Its siblings have one:
  `argmin_cost_order_invariant` (§1.5, `:320`;
  `catalog/packages/Algorithm/Optimization/Argmin.ken.md:159`),
  `argmin_option_cost_order_invariant` (`:244`), and §3.5 and §3.7 for
  argmax.
- **The premise.** Use §1.5's `same_order : (i : Nat) → (j : Nat) →
  Equal Bool (lt_nat (f i) (f j)) (lt_nat i j)` unchanged, so all five
  invariance laws share one premise.
- **The operation.** `least_k` is `take a k (sort a (cost_le a cost)
  xs)` and `cost_le` is `leq_nat (cost x) (cost y)` (`Argmin.ken.md:303`,
  `:305`). `sort` and `insert` are at
  `Data/Collections/Derived.ken.md:1806`, `:1816`.
- **The bridges exist.** `Data.Numeric.Nat.Order` exports
  `leq_not_lt for lt_nat` (`:684`; leq gives reverse lt False) and
  `false_reverse_leq for lt_nat` (`:660`; lt False gives reverse leq).
  `bool_false_of_not_true` is public in `Core.Classes.LawfulClasses`
  (`:485`).
- **Seeds and tests.** The seed sits beside
  `conformance/stdlib/optimization/seed-least-k.md`. The test pattern is
  `crates/ken-elaborator/tests/cat_optimization_argmin_option_cost_order.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §4.4, the catalog law with any private helper, a seed, a
package test and the status lines. No §1 to §4.3 operation or law
type moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-MUL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name, binder order, and whether the
  comparator congruence (`sort` under pointwise-equal comparators) is a
  public `Data.Collections.Derived` law or a private Argmin helper,
  naming its owner.
- **AC-1.** §4.4 states the ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law with
  `f c = add c three` to §4.3's input (labels `A, B, C, D`, costs
  `2, 1, 2, 3`) at `k = 2`, giving `[B, A]`. Its refused block, on a
  fresh accepted prefix and in the same form as an accepted row, claims
  that the constant transform `f c = Zero` also gives `[B, A]`. Stable
  ties make that selection `[A, B]`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond `same_order`.
- A change to an existing §1 to §4.3 operation or law type.

Checks: 4 fired: the obligation was written out: `same_order` with
`leq_not_lt` and `false_reverse_leq`, split on a bound `leq_nat i j`,
gives `Equal Bool (cost_le a (comp a Nat Nat f cost) x y)
(cost_le a cost x y)` pointwise; induction over `insert` and `sort` on
the bound comparator result then gives equal sorted lists, and `cong`
through `take` closes. 3 fired: `least_k` occurs in Argmin, its seed,
its test, spec 72, `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md`;
grep covered `catalog/`, `crates/`, `conformance/`, `spec/` and
`examples/`. 8 fired: a law that drops the premise accepts the AC-2
refused row; the real operation's stable sort under a constant cost
keeps input order, `[A, B]`, so it refuses that row.
